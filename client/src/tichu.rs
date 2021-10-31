use crate::types::CTSMsgInternal;
use anyhow::Error;
use common::{
    clean_up_display_name, clean_up_game_code, get_card_combination, sort_cards_for_hand,
    validate_display_name, validate_game_code, validate_team_name, CTSMsg, CallGrandTichuRequest,
    Card, CardTrade, MutableTeam, OtherPlayerOption, PublicGameStage, PublicGameState, PublicUser,
    STCMsg, TeamOption, TichuCallStatus, NO_USER_ID,
};
use log::*;
use serde_derive::{Deserialize, Serialize};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use yew::format::{Binary, Json};
use yew::prelude::*;
use yew::services::interval::IntervalTask;
use yew::services::storage::{Area, StorageService};
use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};
use yew::services::IntervalService;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum WSConnectionStatus {
    Connected,
    NotConnected,
}

pub struct App {
    link: ComponentLink<Self>,
    interval_task: Option<IntervalTask>,
    ws: Option<WebSocketTask>,
    storage: StorageService,
    state: State,
}

#[derive(Serialize, Deserialize)]
struct State {
    ws_connection_status: WSConnectionStatus,
    user_id: String,
    display_name: String,
    game_state: Option<PublicGameState>,
    is_alive: bool,

    join_room_game_code_input: String,
    display_name_input: String,
    team_a_name_input: String,
    team_b_name_input: String,

    selected_pre_play_card: Option<Card>,
    trade_to_opponent1: Option<Card>,
    trade_to_teammate: Option<Card>,
    trade_to_opponent2: Option<Card>,

    selected_play_cards: Vec<Card>,
}

const USER_ID_STORAGE_KEY: &str = "yew.tichu.user_id";
const DISPLAY_NAME_STORAGE_KEY: &str = "yew.tichu.display_name";

pub enum AppMsg {
    ConnectToWS,
    BeginPing,
    Disconnected,
    Noop,
    WSMsgReceived(Result<Vec<u8>, Error>),
    SendWSMsg(CTSMsgInternal),
    SetUserId(String),
    SetDisplayName(String),
    SetDisplayNameInput(String),
    SetJoinRoomGameCodeInput(String),
    SetTeamANameInput(String),
    SetTeamBNameInput(String),
    SetSelectedPrePlayCard(usize),
    RemoveSelectedPrePlayCard,

    SetTrade(OtherPlayerOption),
    RemoveTrade(OtherPlayerOption),
    AddSelectedPlayCard(usize),
    RemoveSelectedPlayCard(usize),
}

const PING_INTERVAL_MS: u64 = 5000;
static APP_REFERENCE_MUTEX_ERROR: &str = "Could not acquire Mutex lock for app";

/// Wrapper around inherently non-Send/Sync-safe data.
struct StaticMut<T> {
    ptr: Arc<Mutex<Option<T>>>,
}
unsafe impl<T> Send for StaticMut<T> {}
unsafe impl<T> Sync for StaticMut<T> {}

lazy_static! {
    /// HACK: store static reference to app to allow
    /// accessing the component methods from set_interval closures.
    /// Wrapped in an Arc<Mutex> to at least mitigate some data race possibilities.
    static ref APP_REFERENCE: StaticMut<*mut App> = StaticMut {
        ptr: Arc::new(Mutex::new(None)),
};
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut storage =
            StorageService::new(Area::Local).expect("Could not get retrieve StorageService");
        let user_id = if let Json(Ok(restored_user_id)) = storage.restore(USER_ID_STORAGE_KEY) {
            restored_user_id
        } else {
            String::new()
        };

        // default to NO_USER_ID stand-in if string is empty
        let user_id = if user_id.trim().is_empty() {
            storage.store(USER_ID_STORAGE_KEY, Json(&NO_USER_ID));
            String::from(NO_USER_ID)
        } else {
            user_id
        };

        info!("user_id is {}", user_id);

        let display_name =
            if let Json(Ok(restored_display_name)) = storage.restore(DISPLAY_NAME_STORAGE_KEY) {
                restored_display_name
            } else {
                let new_display_name = String::from("");
                storage.store(USER_ID_STORAGE_KEY, Json(&new_display_name));
                new_display_name
            };

        let state = State {
            ws_connection_status: WSConnectionStatus::NotConnected,
            user_id,
            display_name: display_name.clone(),
            display_name_input: display_name,
            game_state: None,
            join_room_game_code_input: "".into(),
            is_alive: false,
            team_a_name_input: "".into(),
            team_b_name_input: "".into(),
            selected_pre_play_card: None,
            trade_to_opponent1: None,
            trade_to_teammate: None,
            trade_to_opponent2: None,
            selected_play_cards: Vec::new(),
        };
        Self {
            interval_task: None,
            ws: None,
            storage,
            link,
            state,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        // connect to websocket on first render
        if self.ws.is_none() && first_render {
            self.link.send_message(AppMsg::ConnectToWS);
            self.link.send_message(AppMsg::BeginPing);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            AppMsg::Noop => false,
            AppMsg::Disconnected => {
                self.ws = None;
                self.state.ws_connection_status = WSConnectionStatus::NotConnected;
                true
            }
            AppMsg::ConnectToWS => {
                info!("Connecting to websocket...");
                let handle_ws_receive_data = self.link.callback(AppMsg::WSMsgReceived);
                let handle_ws_update_status = self.link.callback(|ws_status| {
                    info!("Websocket status: {:?}", ws_status);
                    match ws_status {
                        WebSocketStatus::Closed | WebSocketStatus::Error => AppMsg::Disconnected,
                        WebSocketStatus::Opened => AppMsg::Noop,
                    }
                });
                if self.ws.is_none() {
                    let url = format!("ws://localhost:8080/ws?user_id={}", self.state.user_id);
                    let ws_task = WebSocketService::connect_binary(
                        &url,
                        handle_ws_receive_data,
                        handle_ws_update_status,
                    )
                    .expect("Couldn't initialize websocket connection");
                    self.ws = Some(ws_task);
                    self.state.is_alive = true;
                    self.state.ws_connection_status = WSConnectionStatus::Connected;
                }
                true
            }
            AppMsg::BeginPing => {
                // store a raw pointer to the App component to use in set_interval
                // closure--to enable sending websocket message from the set_interval
                let reference: *mut App = self;
                let mut app_reference_guard =
                    APP_REFERENCE.ptr.lock().expect(APP_REFERENCE_MUTEX_ERROR);
                app_reference_guard.replace(reference);

                let interval_task = IntervalService::spawn(
                    Duration::from_millis(PING_INTERVAL_MS),
                    Callback::Callback(Rc::new(|_| {
                        let app_reference_guard =
                            APP_REFERENCE.ptr.lock().expect(APP_REFERENCE_MUTEX_ERROR);
                        if let Some(app) = *app_reference_guard {
                            unsafe {
                                (*app)
                                    .link
                                    .send_message(AppMsg::SendWSMsg(CTSMsgInternal::Ping));
                            }
                        }
                    })),
                );
                self.interval_task = Some(interval_task);
                false
            }
            AppMsg::SendWSMsg(msg_type) => self.send_ws_message(msg_type),
            AppMsg::WSMsgReceived(data) => self.handle_ws_message_received(data),
            AppMsg::SetUserId(s) => {
                self.storage.store(USER_ID_STORAGE_KEY, Json(&s));
                self.state.user_id = s;
                false
            }
            AppMsg::SetDisplayName(s) => {
                let s = clean_up_display_name(&s);
                self.storage.store(DISPLAY_NAME_STORAGE_KEY, Json(&s));
                self.state.display_name = s.clone();
                self.state.display_name_input = s;
                true
            }
            AppMsg::SetJoinRoomGameCodeInput(s) => {
                let s = clean_up_game_code(&s);
                self.state.join_room_game_code_input = s;
                true
            }
            AppMsg::SetDisplayNameInput(s) => {
                self.state.display_name_input = s;
                true
            }
            AppMsg::SetTeamANameInput(s) => {
                self.state.team_a_name_input = s;
                true
            }
            AppMsg::SetTeamBNameInput(s) => {
                self.state.team_b_name_input = s;
                true
            }
            AppMsg::SetSelectedPrePlayCard(i) => {
                if !self.can_select_pre_play_card() {
                    warn!("Invalid state to set selected card");
                    return false;
                }

                let card_from_hand = if let Some(game_state) = &self.state.game_state {
                    if let Some(card) = game_state.current_user.hand.get(i) {
                        card.clone()
                    } else {
                        warn!("Couldn't find index {:?} in current users hand", i);
                        return false;
                    }
                } else {
                    warn!("Can't SetSelectedPrePlayCard because current game_state is None");
                    return false;
                };

                if self.is_card_is_set_to_trade(&card_from_hand) {
                    warn!("Can't set selected card since card is already set to trade");
                    return false;
                }

                self.state.selected_pre_play_card.replace(card_from_hand);
                true
            }
            AppMsg::RemoveSelectedPrePlayCard => {
                self.state.selected_pre_play_card = None;
                true
            }
            AppMsg::SetTrade(trade_to_person) => {
                if !self.can_set_trade() {
                    warn!("Invalid state to set trade to {:?}", trade_to_person);
                    return false;
                }
                if let Some(selected_pre_play_card) = &self.state.selected_pre_play_card {
                    if self.is_card_is_set_to_trade(selected_pre_play_card) {
                        warn!(
                            "Can't set trade to {:?} since card is already set to trade",
                            trade_to_person
                        );
                        return false;
                    }
                }
                match &trade_to_person {
                    OtherPlayerOption::Opponent1 => {
                        self.state.trade_to_opponent1 = self.state.selected_pre_play_card.take();
                    }
                    OtherPlayerOption::Teammate => {
                        self.state.trade_to_teammate = self.state.selected_pre_play_card.take();
                    }
                    OtherPlayerOption::Opponent2 => {
                        self.state.trade_to_opponent2 = self.state.selected_pre_play_card.take();
                    }
                }

                if let Some(game_state) = &mut self.state.game_state {
                    sort_cards_for_hand(&mut game_state.current_user.hand);
                }

                true
            }
            AppMsg::RemoveTrade(trade_to_person) => {
                match &trade_to_person {
                    OtherPlayerOption::Opponent1 => {
                        self.state.trade_to_opponent1 = None;
                    }
                    OtherPlayerOption::Teammate => {
                        self.state.trade_to_teammate = None;
                    }
                    OtherPlayerOption::Opponent2 => {
                        self.state.trade_to_opponent2 = None;
                    }
                }

                if let Some(game_state) = &mut self.state.game_state {
                    sort_cards_for_hand(&mut game_state.current_user.hand);
                }

                true
            }
            AppMsg::AddSelectedPlayCard(i) => {
                if !self.can_select_play_card() {
                    warn!("Invalid state to add selected play card");
                    return false;
                }

                let card_from_hand = if let Some(game_state) = &self.state.game_state {
                    if let Some(card) = game_state.current_user.hand.get(i) {
                        card.clone()
                    } else {
                        warn!("Couldn't find index {:?} in current users hand", i);
                        return false;
                    }
                } else {
                    warn!("Can't AddSelectedPlayCard because current game_state is None");
                    return false;
                };

                if self.is_play_card_selected(&card_from_hand) {
                    warn!("Can't set selected card since card is already selected to play");
                    return false;
                }

                info!("Adding {:?} to selected_play_cards", card_from_hand);
                self.state.selected_play_cards.push(card_from_hand);
                sort_cards_for_hand(&mut self.state.selected_play_cards);
                true
            }
            AppMsg::RemoveSelectedPlayCard(i) => {
                if let Some(card) = self.state.selected_play_cards.get(i) {
                    info!("Removing {:?} from selected_play_cards", card);
                    self.state.selected_play_cards.remove(i);
                    sort_cards_for_hand(&mut self.state.selected_play_cards);
                }
                true
            }
        }
    }

    fn change(&mut self, _prop: Self::Properties) -> ShouldRender {
        false
    }

    fn destroy(&mut self) {
        // clean up static reference to app
        let mut app_reference_guard = APP_REFERENCE.ptr.lock().expect(APP_REFERENCE_MUTEX_ERROR);
        app_reference_guard.take();
    }

    fn view(&self) -> Html {
        html! {
                <div>
                    {match &self.state.game_state {
                        None => self.view_join(),
                        Some(game_state) =>{
                            match game_state.stage {
                                PublicGameStage::Lobby => self.view_lobby(),
                                PublicGameStage::Teams(_) => self.view_teams(),
                                PublicGameStage::GrandTichu(_) => self.view_grand_tichu(),
                                PublicGameStage::Trade(_) => self.view_trade(),
                                PublicGameStage::Play(_) => self.view_play(),
                                _ => html!{<> </>}
                        }
                    }
                }}
                    <br />
                    <br />
                    <hr />
                    <br />
                    {self.view_debug()}
                </div>
        }
    }
}

impl App {
    fn can_create_game(&self) -> bool {
        self.ws.is_some() && validate_display_name(&self.state.display_name_input).is_none()
    }

    fn can_join_game(&self) -> bool {
        self.ws.is_some()
            && validate_display_name(&self.state.display_name_input).is_none()
            && validate_game_code(&self.state.join_room_game_code_input).is_none()
    }

    fn can_leave_game(&self) -> bool {
        self.ws.is_some()
            && self.state.game_state.is_some()
            && matches!(
                self.state.game_state.as_ref().unwrap().stage,
                PublicGameStage::Lobby
            )
    }

    fn can_call_or_decline_grand_tichu(&self) -> bool {
        if let Some(game_state) = &self.state.game_state {
            if let PublicGameStage::GrandTichu(grand_tichu_stage) = &game_state.stage {
                grand_tichu_stage.grand_tichus.iter().any(|call_status| {
                    *call_status.user_id == self.state.user_id
                        && matches!(call_status.tichu_call_status, TichuCallStatus::Undecided)
                })
            } else {
                false
            }
        } else {
            false
        }
    }

    fn can_call_small_tichu(&self) -> bool {
        // hasn't yet played a card
        let has_played_first_card = if let Some(game_state) = &self.state.game_state {
            game_state.current_user.has_played_first_card
        } else {
            false
        };

        let game_stage_is_active = if let Some(game_state) = &self.state.game_state {
            game_state.stage.is_active()
        } else {
            false
        };

        let undecided_about_small_tichu = if let Some(game_state) = &self.state.game_state {
            let small_tichus = match &game_state.stage {
                PublicGameStage::GrandTichu(public_grand_tichu) => &public_grand_tichu.small_tichus,
                PublicGameStage::Trade(trade) => &trade.small_tichus,
                PublicGameStage::Play(play) => &play.small_tichus,
                // game is not active, therefore can't call small tichu
                _ => return false,
            };

            if let Some(call_status) = small_tichus.iter().find(|user_id_with_tichu_call_status| {
                *user_id_with_tichu_call_status.user_id == *self.state.user_id
            }) {
                matches!(call_status.tichu_call_status, TichuCallStatus::Undecided)
            } else {
                // user's call status not found, therefore undecided
                true
            }
        } else {
            // GameState is None, therefore user is Undecided
            true
        };

        !has_played_first_card && undecided_about_small_tichu && game_stage_is_active
    }

    fn is_team_stage(&self) -> bool {
        match &self.state.game_state {
            None => false,
            Some(game_state) => matches!(&game_state.stage, PublicGameStage::Teams(_)),
        }
    }

    fn is_on_team_a(&self) -> bool {
        match &self.state.game_state {
            None => false,
            Some(game_state) => match &game_state.stage {
                PublicGameStage::Teams(teams) => teams[0]
                    .user_ids
                    .iter()
                    .any(|participant_id| **participant_id == *self.state.user_id),
                _ => false,
            },
        }
    }

    fn is_on_team_b(&self) -> bool {
        match &self.state.game_state {
            None => false,
            Some(game_state) => match &game_state.stage {
                PublicGameStage::Teams(teams) => teams[1]
                    .user_ids
                    .iter()
                    .any(|participant_id| **participant_id == *self.state.user_id),
                _ => false,
            },
        }
    }

    fn view_participants(&self) -> Html {
        if let Some(game_state) = &self.state.game_state {
            html! {
                    <ul>
                    {for game_state.participants.iter().map(|user| {
                        html! {
                            <li>{&user.display_name}</li>
                    }
                })}
                    </ul>
            }
        } else {
            html! {}
        }
    }

    fn get_participant_by_id(&self, user_id: &str) -> Option<&PublicUser> {
        match &self.state.game_state {
            None => None,
            Some(game_state) => game_state
                .participants
                .iter()
                .find(|participant| participant.user_id == user_id),
        }
    }

    fn debug_team(&self, team: &MutableTeam) -> Html {
        html! {
                <ul>
                    <li>{"Team Name: "} {{&team.team_name}}</li>
                    <li>{"Score: "} {{&team.score}}</li>
                    <li>{"Users: "} {for team.user_ids.iter().map(|id| {
                        html!{
                            <p>{match &self.get_participant_by_id(id) {
                                Some(participant) => &participant.display_name,
                                None => ""
                        }}</p>
                    }
                })}</li>
                </ul>
        }
    }

    fn debug_teams(&self) -> Html {
        if let Some(game_state) = &self.state.game_state {
            match &game_state.stage {
                PublicGameStage::Teams(team_state) => {
                    html! {
                            <ul>
                                <li>{self.debug_team(&team_state[0])}</li>
                                <li>{self.debug_team(&team_state[1])}</li>
                            </ul>
                    }
                }
                _ => html! {},
            }
        } else {
            html! {}
        }
    }

    fn debug_owner(&self) -> Html {
        if let Some(game_state) = &self.state.game_state {
            let owner = game_state
                .participants
                .iter()
                .find(|user| user.user_id == game_state.owner_id);

            match owner {
                Some(owner) => {
                    html! {
                            <ul>
                                <li>{&owner.display_name}</li>
                            </ul>
                    }
                }
                None => {
                    html! {
                            <> </>
                    }
                }
            }
        } else {
            html! {}
        }
    }

    fn view_debug_grand_tichu_for_user(&self, user_id: &str, display_name: &str) -> Html {
        let grand_tichu_call_status = if let Some(game_state) = &self.state.game_state {
            let grand_tichus = match &game_state.stage {
                PublicGameStage::GrandTichu(grand_tichu_state) => &grand_tichu_state.grand_tichus,
                PublicGameStage::Trade(trade) => &trade.grand_tichus,
                PublicGameStage::Play(play) => &play.grand_tichus,
                _ => {
                    return html! {
                        <p>{&format!("Grand Tichu Call Status for {display_name}: n/a \n")}</p>
                    }
                }
            };

            match grand_tichus.iter().find(|user_id_with_tichu_call_status| {
                *user_id_with_tichu_call_status.user_id == *user_id
            }) {
                Some(user_id_with_tichu_call_status) => {
                    match user_id_with_tichu_call_status.tichu_call_status {
                        TichuCallStatus::Undecided => "Undecided",
                        TichuCallStatus::Called => "Called",
                        TichuCallStatus::Declined => "Declined",
                        TichuCallStatus::Achieved => "Achieved",
                        TichuCallStatus::Failed => "Failed",
                    }
                }
                None => "n/a",
            }
        } else {
            "n/a"
        };
        html! {
                <p>{&format!("Grand Tichu Call Status for {display_name}: {grand_tichu_call_status} \n")}</p>
        }
    }

    fn view_debug_small_tichu_for_user(&self, user_id: &str, display_name: &str) -> Html {
        let small_tichu_call_status = if let Some(game_state) = &self.state.game_state {
            let small_tichus = match &game_state.stage {
                PublicGameStage::GrandTichu(grand_tichu_state) => &grand_tichu_state.small_tichus,
                PublicGameStage::Trade(trade) => &trade.small_tichus,
                PublicGameStage::Play(play) => &play.small_tichus,
                _ => {
                    return html! {
                        <p>{&format!("Small Tichu Call Status for {display_name}: n/a \n")}</p>
                    }
                }
            };

            match small_tichus.iter().find(|user_id_with_tichu_call_status| {
                *user_id_with_tichu_call_status.user_id == *user_id
            }) {
                Some(user_id_with_tichu_call_status) => {
                    match user_id_with_tichu_call_status.tichu_call_status {
                        TichuCallStatus::Undecided => "Undecided",
                        TichuCallStatus::Called => "Called",
                        TichuCallStatus::Declined => "Declined",
                        TichuCallStatus::Achieved => "Achieved",
                        TichuCallStatus::Failed => "Failed",
                    }
                }
                None => "n/a",
            }
        } else {
            "n/a"
        };
        html! {
                <p>{&format!("Small Tichu Call Status for {display_name}: {small_tichu_call_status}\n")}</p>
        }
    }

    fn view_debug_all_participants_grand_tichu(&self) -> Html {
        match &self.state.game_state {
            Some(game_state) => html! {{
                for game_state
                    .participants
                    .iter()
                    .map(|user| self.view_debug_grand_tichu_for_user(&user.user_id, &user.display_name))
            }},
            None => html! {<> </>},
        }
    }

    fn view_debug_all_participants_small_tichu(&self) -> Html {
        match &self.state.game_state {
            Some(game_state) => html! {{
                for game_state
                    .participants
                    .iter()
                    .map(|user| self.view_debug_small_tichu_for_user(&user.user_id, &user.display_name))
            }},
            None => html! {<> </>},
        }
    }

    fn view_debug(&self) -> Html {
        html! {
                <>
                <h1>{"Debug Info:"}</h1>
                    <p>{"Display Name: "} {&self.state.display_name}</p>
                    <p>{"User ID: "} {&self.state.user_id}</p>
                    <p>{"Websocket Status: "}{match &self.state.ws_connection_status {
                        WSConnectionStatus::Connected => "Connected",
                        WSConnectionStatus::NotConnected => "Not Connected"
                }}</p>
                    <p>{"Game Code: "} {
                        if let Some(game_state) = &self.state.game_state {
                            &game_state.game_code
                    } else {
                            ""
                    }}
                    </p>
                    <p>{"Participants: "} {self.view_participants()}</p>
                    <p>{"Owner: "} {self.debug_owner()}</p>
                    <p>{"Teams: "} {self.debug_teams()}</p>
                    <h2>{"Grand Tichus: "}</h2>
                    {self.view_debug_all_participants_grand_tichu()}
                    <br />
                    <h2>{"Small Tichus: "}</h2>
                    {self.view_debug_all_participants_small_tichu()}
                    <button onclick=self.link.callback(|_| AppMsg::SendWSMsg(CTSMsgInternal::Test))>{"Send test message to server"}</button>
                    <br />
                    <button onclick=self.link.callback(|_| AppMsg::SendWSMsg(CTSMsgInternal::Ping))>{"Send ping to server"}</button>
                </>
        }
    }

    fn view_join(&self) -> Html {
        html! {
                <>
                <h1>{"Tichu"}</h1>
                    <form onsubmit=self.link.callback(|e: FocusEvent| {
                        e.prevent_default();
                        AppMsg::SendWSMsg(CTSMsgInternal::JoinGameWithGameCode)
                })>
                        <label for="join-room-display-name-input">{"Display Name"}</label>
                        <br />
                        <input
                            id="join-room-display-name-input"
                            type="text"
                            value=self.state.display_name_input.clone()
                            oninput=self.link.callback(|e: InputData| AppMsg::SetDisplayNameInput(e.value))/>
                        <br />
                        <label for="join-room-game-code-input">{"Game Code"}</label>
                        <br />
                        <input
                            id="join-room-game-code-input"
                            type="text"
                            value=self.state.join_room_game_code_input.clone()
                            oninput=self.link.callback(|e: InputData| AppMsg::SetJoinRoomGameCodeInput(e.value))/>
                        <br />
                        <button
                            type="submit"
                            disabled=!self.can_join_game()
                            >{"Join game"}</button>
                    </form>
                    <br />
                    <br />
                    <form onsubmit=self.link.callback(|e: FocusEvent| {
                        e.prevent_default();
                        AppMsg::SendWSMsg(CTSMsgInternal::CreateGame)
                })>
                        <label for="join-room-display-name-input">{"Display Name"}</label>
                        <br />
                        <input
                            id="create-room-display-name-input"
                            type="text"
                            value=self.state.display_name_input.clone()
                            oninput=self.link.callback(|e: InputData| AppMsg::SetDisplayNameInput(e.value))/>
                        <br />
                        <button
                            type="submit"
                            disabled=!self.can_create_game()
                            >{"Create game"}</button>
                    </form>
                </>
        }
    }

    fn view_lobby(&self) -> Html {
        html! {
                <>
                    <h1>{"Lobby"}</h1>
                    <h2>{"Game Code: "} {
                        if let Some(game_state) = &self.state.game_state {
                            &game_state.game_code
                    } else {
                            ""
                    }
                }
                    </h2>
                    <h3>{"Joined:"}</h3>
                    <br />
                    {self.view_participants()}
                    <button
                        onclick=self.link.callback(|_| {AppMsg::SendWSMsg(CTSMsgInternal::LeaveGame)})
                        disabled=!self.can_leave_game()
                        >
                        {"Leave game"}
                    </button>
                </>
        }
    }

    fn is_current_user_owner(&self) -> bool {
        match &self.state.game_state {
            None => false,
            Some(game_state) => game_state.owner_id == self.state.user_id,
        }
    }

    fn can_start_game(&self) -> bool {
        let current_user_is_owner = self.is_current_user_owner();
        let mut teams_are_ready = false;
        if let Some(game_state) = &self.state.game_state {
            if let PublicGameStage::Teams(teams_state) = &game_state.stage {
                if teams_state[0].user_ids.len() == 2 && teams_state[1].user_ids.len() == 2 {
                    teams_are_ready = true;
                }
            }
        }
        current_user_is_owner && teams_are_ready
    }

    fn view_teams(&self) -> Html {
        html! {
                <>
                <h1>{"Teams"}</h1>
                    <label for="team-a-name-input">{"Team Name"}</label>
                    <br />
                    <form onsubmit=self.link.callback(|e: FocusEvent | {
                        e.prevent_default();
                        AppMsg::SendWSMsg(CTSMsgInternal::RenameTeam(TeamOption::TeamA))
                })>
                    <label for="team-a-name-input">{"Team Name"}</label>
                    <input
                        id="team-a-name-input"
                        disabled=!self.is_team_stage() || self.is_on_team_b()
                        type="text"
                        value=self.state.team_a_name_input.clone()
                        oninput=self.link.callback(|e: InputData| AppMsg::SetTeamANameInput(e.value))/>
                   </form>
                    <br />
                    <button
                        onclick=self.link.callback(|_| {AppMsg::SendWSMsg(CTSMsgInternal::MoveToTeam(TeamOption::TeamA))})
                        disabled=!self.is_team_stage() || self.is_on_team_a()
                        >{"Move to Team A"}</button>
                    <br />
                    <button
                        onclick=self.link.callback(|_| {AppMsg::SendWSMsg(CTSMsgInternal::MoveToTeam(TeamOption::TeamB))})
                        disabled=!self.is_team_stage() || self.is_on_team_b()
                        >{"Move to Team B"}</button>
                    <br />
                    <br />
                    <form onsubmit=self.link.callback(|e: FocusEvent | {
                        e.prevent_default();
                        AppMsg::SendWSMsg(CTSMsgInternal::RenameTeam(TeamOption::TeamB))
                })>
                        <label for="team-b-name-input">{"Team Name"}</label>
                        <input
                            id="team-b-name-input"
                            disabled=!self.is_team_stage() || self.is_on_team_a()
                            type="text"
                            value=self.state.team_b_name_input.clone()
                            oninput=self.link.callback(|e: InputData| AppMsg::SetTeamBNameInput(e.value))/>
                   </form>
                   {if self.is_current_user_owner() {
                      html!{
                        <button
                            onclick=self.link.callback(|_| {AppMsg::SendWSMsg(CTSMsgInternal::StartGrandTichu)})
                            disabled=!self.can_start_game()
                        >{"Start"}</button>
                    }
                } else {
                       html!{
                           <> </>
                    }
                }}
                </>
        }
    }

    fn is_card_is_set_to_trade(&self, card: &Card) -> bool {
        let card_is_set_to_trade_opponent1 =
            if let Some(card_to_trade) = &self.state.trade_to_opponent1 {
                *card_to_trade == *card
            } else {
                false
            };

        let card_is_set_to_trade_teammate =
            if let Some(card_to_trade) = &self.state.trade_to_teammate {
                *card_to_trade == *card
            } else {
                false
            };

        let card_is_set_to_trade_opponent2 =
            if let Some(card_to_trade) = &self.state.trade_to_opponent2 {
                *card_to_trade == *card
            } else {
                false
            };

        card_is_set_to_trade_opponent1
            || card_is_set_to_trade_teammate
            || card_is_set_to_trade_opponent2
    }

    fn view_pre_play_hand(&self) -> Html {
        if let Some(game_state) = &self.state.game_state {
            html! {
                    <ul>
                        {for game_state.current_user.hand.iter().enumerate().map(|(i, card)| {
                            // do not render card if the stage is Trade and it is currently selected
                            // OR if it has been selected for trade with opponent
                        let card_is_selected = if let Some(selected_pre_play_card) = &self.state.selected_pre_play_card {
                            *selected_pre_play_card == *card
                        } else {
                                false
                        };
                        if (card_is_selected || self.is_card_is_set_to_trade(card)) && self.stage_is_trade() {
                            html!{}
                        } else {
                                html!{
                                    <li>
                                        <button
                                            disabled=!self.can_select_pre_play_card()
                                            onclick=self.link.callback(move |_| {AppMsg::SetSelectedPrePlayCard(i)})
                                            >
                                            {&format!("{:#?}", card)}
                                        </button>
                                    </li>
                            }
                        }
                    })}
                    </ul>
            }
        } else {
            html! {}
        }
    }

    fn call_small_tichu_button(&self) -> Html {
        html! {
                <button
                onclick=self.link.callback(|_| {AppMsg::SendWSMsg(CTSMsgInternal::CallSmallTichu)})
                disabled=!self.can_call_small_tichu()
                >{"Call Small Tichu"}</button>
        }
    }

    fn view_grand_tichu_status_for_current_user(&self) -> Html {
        let grand_tichu_call_status = match &self.state.game_state {
            Some(game_state) => match &game_state.stage {
                PublicGameStage::GrandTichu(grand_tichu_state) => {
                    match grand_tichu_state.grand_tichus.iter().find(
                        |user_id_with_tichu_call_status| {
                            *user_id_with_tichu_call_status.user_id == *self.state.user_id
                        },
                    ) {
                        Some(user_id_with_tichu_call_status) => {
                            match user_id_with_tichu_call_status.tichu_call_status {
                                TichuCallStatus::Undecided => "Undecided",
                                TichuCallStatus::Called => "Called",
                                TichuCallStatus::Declined => "Declined",
                                TichuCallStatus::Achieved => "Achieved",
                                TichuCallStatus::Failed => "Failed",
                            }
                        }
                        None => "n/a",
                    }
                }
                _ => "n/a",
            },
            None => "n/a",
        };
        html! {
                <p>{"Grand Tichu Call Status: "} {grand_tichu_call_status} {"\n"}</p>
        }
    }

    fn view_grand_tichu(&self) -> Html {
        html! {
                <>
                    <h1>{"Grand Tichu"}</h1>
                    {self.view_grand_tichu_status_for_current_user()}
                    <button
                        onclick=self.link.callback(|_| {AppMsg::SendWSMsg(CTSMsgInternal::CallGrandTichu(CallGrandTichuRequest::Call))})
                        disabled=!self.can_call_or_decline_grand_tichu()
                    >{"Call Grand Tichu"}</button>
                    <button
                        onclick=self.link.callback(|_| {AppMsg::SendWSMsg(CTSMsgInternal::CallGrandTichu(CallGrandTichuRequest::Decline))})
                        disabled=!self.can_call_or_decline_grand_tichu()
                    >{"Decline Grand Tichu"}</button>
                    {self.call_small_tichu_button()}
                    <p>{"Hand:"}</p>
                    {self.view_pre_play_hand()}
                </>
        }
    }

    fn stage_is_trade(&self) -> bool {
        matches!(
            self.state.game_state.as_ref().unwrap().stage,
            PublicGameStage::Trade(_)
        )
    }

    fn stage_is_play(&self) -> bool {
        matches!(
            self.state.game_state.as_ref().unwrap().stage,
            PublicGameStage::Play(_)
        )
    }

    /// only for use in the Trade stage
    fn can_select_pre_play_card(&self) -> bool {
        self.stage_is_trade() && !self.has_submitted_trade()
    }

    /// only for use in the Play stage
    fn can_select_play_card(&self) -> bool {
        self.stage_is_play()
    }

    fn view_selected_pre_play_card_button(&self) -> Html {
        match &self.state.selected_pre_play_card {
            Some(card) => {
                html! {
                        <button
                            onclick=self.link.callback(|_| {AppMsg::RemoveSelectedPrePlayCard})
                            disabled=self.state.selected_pre_play_card.is_none()
                            type="button">
                            {&format!("Remove {:#?}", card)}
                        </button>
                }
            }
            None => html! {
                <button
                    disabled=true
                    type="button">
                    {"No card selected"}
                </button>
            },
        }
    }

    fn can_set_trade(&self) -> bool {
        matches!(&self.state.selected_pre_play_card, Some(_))
    }

    fn can_remove_trade(&self, trade_to_person: &OtherPlayerOption) -> bool {
        match trade_to_person {
            OtherPlayerOption::Opponent1 => &self.state.trade_to_opponent1,
            OtherPlayerOption::Teammate => &self.state.trade_to_teammate,
            OtherPlayerOption::Opponent2 => &self.state.trade_to_opponent2,
        }
        .is_some()
    }

    fn can_submit_trade(&self) -> bool {
        // state is trade
        if let Some(game_state) = &self.state.game_state {
            if let PublicGameStage::Trade(_) = &game_state.stage {
                // there is a card assigned to every user
                self.state.trade_to_opponent1.is_some()
                    && self.state.trade_to_teammate.is_some()
                    && self.state.trade_to_opponent2.is_some()
            } else {
                false
            }
        } else {
            false
        }
    }

    fn view_trade_to_person(&self, trade_to_person: OtherPlayerOption) -> Html {
        let trade_to_person_state = match &trade_to_person {
            OtherPlayerOption::Opponent1 => &self.state.trade_to_opponent1,
            OtherPlayerOption::Teammate => &self.state.trade_to_teammate,
            OtherPlayerOption::Opponent2 => &self.state.trade_to_opponent2,
        };

        let trade_to_person_display_name = {
            let public_other_players = &self.state.game_state.as_ref().unwrap().get_other_players();
            let display_name = if let Some(public_other_players) = public_other_players {
                match &trade_to_person {
                    OtherPlayerOption::Opponent1 => {
                        public_other_players.opponent_1.display_name.clone()
                    }
                    OtherPlayerOption::Teammate => {
                        public_other_players.teammate.display_name.clone()
                    }
                    OtherPlayerOption::Opponent2 => {
                        public_other_players.opponent_2.display_name.clone()
                    }
                }
            } else {
                String::new()
            };
            display_name
        };

        return html! {
              <>
                {if self.state.selected_pre_play_card.is_none() {
                    html!{
                        <button
                            disabled=!self.can_remove_trade(&trade_to_person)
                            onclick=self.link.callback(move |_| {AppMsg::RemoveTrade(trade_to_person.clone())})
                        >
                        {match trade_to_person_state {
                            Some(card) => format!("Remove {:?} to {}", card, trade_to_person_display_name),
                            None => format!("No trade selected for {}", trade_to_person_display_name),
                        }}
                        </button>
                    }
                } else {
                        html!{
                            <button onclick=self.link.callback(move |_| {AppMsg::SetTrade(trade_to_person.clone())})>
                                {if trade_to_person_state.is_some() {
                                    format!("Replace trade with {}", trade_to_person_display_name)
                                } else {
                                        format!("Send to {}", trade_to_person_display_name)
                                }}
                            </button>
                    }
                }}
              </>
        };
    }

    fn has_submitted_trade(&self) -> bool {
        if let Some(game_state) = &self.state.game_state {
            if let PublicGameStage::Trade(trade_state) = &game_state.stage {
                trade_state
                    .submitted_trades
                    .iter()
                    .any(|user_id| *user_id == *self.state.user_id)
            } else {
                false
            }
        } else {
            false
        }
    }

    fn view_trade(&self) -> Html {
        html! {
                <>
                    <h1>{"Trade"}</h1>
                    <p>{&format!("Has submitted trade: {:?}", self.has_submitted_trade())}</p>
                    {if !self.has_submitted_trade() {
                        html!{
                            <>
                                <button
                                    onclick=self.link.callback(|_| {AppMsg::SendWSMsg(CTSMsgInternal::SubmitTrade)})
                                    disabled=!self.can_submit_trade()
                                    type="submit">
                                {"Submit"}
                                </button>
                                <br />
                                <br />
                                {self.view_trade_to_person(OtherPlayerOption::Opponent1)}
                                <br />
                                {self.view_trade_to_person(OtherPlayerOption::Teammate)}
                                <br />
                                {self.view_trade_to_person(OtherPlayerOption::Opponent2)}
                                <br />
                                <br />
                                <br />
                                {self.call_small_tichu_button()}
                                <br />
                                <br />
                                <br />
                                {self.view_selected_pre_play_card_button()}
                                <br />
                                <br />
                                <br />
                            </>
                    }
                } else {
                        html!{
                            <p>{"Waiting for others to trade..."}</p>
                    }
                }}
                    {self.view_pre_play_hand()}
                </>
        }
    }

    fn is_play_card_selected(&self, card: &Card) -> bool {
        self.state
            .selected_play_cards
            .iter()
            .any(|selected_card| *selected_card == *card)
    }

    fn is_valid_combination(&self) -> bool {
        get_card_combination(&self.state.selected_play_cards).is_some()
    }

    fn view_play_hand(&self) -> Html {
        if let Some(game_state) = &self.state.game_state {
            html! {
                    <ul>
                        {for game_state.current_user.hand.iter().enumerate().map(|(i, card)| {
                            // do not render card if the stage is Trade and it is currently selected
                            // OR if it has been selected for trade with opponent
                        let card_is_selected = self.state.selected_play_cards.iter().any(|selected_card| {
                            *selected_card == *card
                        });
                        if card_is_selected {
                            html!{}
                        } else {
                                html!{
                                    <li>
                                        <button
                                            disabled=!self.can_select_play_card()
                                            onclick=self.link.callback(move |_| {AppMsg::AddSelectedPlayCard(i)})
                                            >
                                            {&format!("{:#?}", card)}
                                        </button>
                                    </li>
                            }
                        }
                    })}
                    </ul>
            }
        } else {
            html! {}
        }
    }

    fn view_selected_play_card_buttons(&self) -> Html {
        html! {
            <>
            { for self.state.selected_play_cards.iter().enumerate().map(|(i, selected_card)| {
                 html! {
                     <button
                         onclick=self.link.callback(move |_| {AppMsg::RemoveSelectedPlayCard(i)})
                         type="button">
                         {&format!("Remove {:#?}", selected_card)}
                     </button>
                 }
            })}
            </>
        }
    }

    fn view_turn_display_name(&self) -> Html {
        if let Some(game_state) = &self.state.game_state {
            if let PublicGameStage::Play(play_state) = &game_state.stage {
                let turn_user_id = &play_state.turn_user_id;
                let turn_user = &game_state.get_user_by_user_id(&turn_user_id);
                if let Some(turn_user) = turn_user {
                    return html! {
                        <p>{&format!("Current turn: {}", turn_user.display_name)}</p>
                    };
                }
            }
        }

        html! {}
    }

    fn view_play(&self) -> Html {
        html! {
              <>
                <h1>{"Play"}</h1>
                <br />
                <br />
                {self.view_turn_display_name()}
                <br />
                <br />
                <button
                    disabled=!self.is_valid_combination()
                    type="submit"
                    >
                    {"Submit cards"}
                </button>
                <br />
                <br />
                {self.call_small_tichu_button()}
                <br />
                <br />
                {self.view_selected_play_card_buttons()}
                <br />
                <br />
                {self.view_play_hand()}
              </>
        }
    }

    /// Handles when a websocket message is received from the server
    /// Returns whether the component should re-render or not
    fn handle_ws_message_received(&mut self, data: Result<Vec<u8>, Error>) -> bool {
        let mut should_rerender = true;
        if data.is_err() {
            error!("Data received from websocket was an error {:?}", &data);
            return false;
        }
        let data: Option<STCMsg> = bincode::deserialize(&data.unwrap()).ok();
        info!("Received websocket message: {:?}", &data);
        match data {
            None => {
                warn!("Deserialized data is None. This probably indicates there was an error deserializing the websocket message binary");
            }
            Some(data) => match data {
                STCMsg::Ping => {
                    self.link
                        .send_message(AppMsg::SendWSMsg(CTSMsgInternal::Pong));
                }
                STCMsg::UserIdAssigned(s) => {
                    self.link.send_message(AppMsg::SetUserId(s));
                }
                STCMsg::GameState(new_game_state) => {
                    let new_game_state = *new_game_state;

                    // if team names are empty, update team name inputs to reflect state
                    if self.state.team_a_name_input.is_empty()
                        || self.state.team_b_name_input.is_empty()
                    {
                        if let Some(new_game_state) = &new_game_state {
                            if let PublicGameStage::Teams(teams_state) = &new_game_state.stage {
                                self.link.send_message_batch(vec![
                                    AppMsg::SetTeamANameInput(
                                        (*teams_state[0].team_name).to_string(),
                                    ),
                                    AppMsg::SetTeamBNameInput(
                                        (*teams_state[1].team_name).to_string(),
                                    ),
                                ])
                            }
                        }
                    }

                    // move into block and back out for mutability (TODO: clean up later)
                    let new_game_state = if let Some(mut new_game_state) = new_game_state {
                        // save display name input to state/localStorage
                        self.link.send_message(AppMsg::SetDisplayName(
                            (*new_game_state.current_user.display_name).to_string(),
                        ));

                        // sort current user's hand
                        sort_cards_for_hand(&mut new_game_state.current_user.hand);

                        Box::new(Some(new_game_state))
                    } else {
                        Box::new(new_game_state)
                    };

                    self.state.game_state = *new_game_state;
                    should_rerender = true;
                }
                STCMsg::UnexpectedMessageReceived(s) => {
                    warn!(
                        "Server received unexpected message from client. Message sent from client: {}",
                        s
                    );
                }
                STCMsg::Pong => {
                    self.state.is_alive = true;
                }
                STCMsg::TeamARenamed(new_team_a_name) => {
                    self.link
                        .send_message(AppMsg::SetTeamANameInput(new_team_a_name));
                }
                STCMsg::TeamBRenamed(new_team_b_name) => {
                    self.link
                        .send_message(AppMsg::SetTeamBNameInput(new_team_b_name));
                }
                STCMsg::Test(_) => {}
                STCMsg::GameCreated { .. } => {}
                STCMsg::UserJoined(_) => {}
                STCMsg::UserDisconnected(_) => {}
                STCMsg::UserReconnected(_) => {}
                STCMsg::UserLeft(_) => {}
                STCMsg::OwnerReassigned(_) => {}
                STCMsg::UserMovedToTeamA(_) => {}
                STCMsg::UserMovedToTeamB(_) => {}
                STCMsg::GameStageChanged(_) => {}
                STCMsg::GrandTichuCalled(_, _) => {}
                STCMsg::SmallTichuCalled(_) => {}
                STCMsg::TradeSubmitted(_) => {}
                _ => warn!("Unexpected websocket message received {:#?}", data),
            },
        }

        should_rerender
    }

    /// Sends a message to the server via websocket
    /// Returns whether the component should rerender
    fn send_ws_message(&mut self, msg_type: CTSMsgInternal) -> bool {
        info!("Sending websocket message: {:#?}", msg_type);
        match msg_type {
            CTSMsgInternal::Test => {
                self._send_ws_message(&CTSMsg::Test(String::from("Hello, server!")));
                false
            }
            CTSMsgInternal::Ping => {
                let mut should_reconnect = false;
                if self.ws.is_none() {
                    info!("Trying to ping, but there is no websocket connection. Attempting to reconnect");
                    should_reconnect = true;
                } else if !self.state.is_alive {
                    info!("Server isn't responding to pings. Closing websocket connection and attempting to reconnect.");
                    should_reconnect = true;
                    let ws = self.ws.take();
                    drop(ws);
                }

                self.state.is_alive = false;
                if should_reconnect {
                    self.link.send_message(AppMsg::ConnectToWS);
                } else {
                    self._send_ws_message(&CTSMsg::Ping);
                }
                false
            }
            CTSMsgInternal::Pong => {
                self._send_ws_message(&CTSMsg::Pong);
                false
            }
            CTSMsgInternal::CreateGame => {
                if !self.can_create_game() {
                    return false;
                }

                let msg = CTSMsg::CreateGame {
                    user_id: self.state.user_id.clone(),
                    display_name: self.state.display_name_input.clone(),
                };

                self._send_ws_message(&msg);
                false
            }
            CTSMsgInternal::JoinGameWithGameCode => {
                if !self.can_join_game() {
                    return false;
                }

                let msg = CTSMsg::JoinGameWithGameCode {
                    game_code: self.state.join_room_game_code_input.clone().to_uppercase(),
                    display_name: self.state.display_name_input.clone(),
                    user_id: self.state.user_id.clone(),
                };

                self._send_ws_message(&msg);
                false
            }
            CTSMsgInternal::LeaveGame => {
                if !self.can_leave_game() {
                    return false;
                }

                self._send_ws_message(&CTSMsg::LeaveGame);
                false
            }
            CTSMsgInternal::MoveToTeam(team_option) => {
                self._send_ws_message(&CTSMsg::MoveToTeam(team_option));
                false
            }
            CTSMsgInternal::RenameTeam(team_option) => {
                let team_name_input_clone = match &team_option {
                    TeamOption::TeamA => self.state.team_a_name_input.clone(),
                    TeamOption::TeamB => self.state.team_b_name_input.clone(),
                };

                let team_index = match &team_option {
                    TeamOption::TeamA => 0,
                    TeamOption::TeamB => 1,
                };

                // if team name input is empty on blur, replace with existing state and do not try to update on server
                if team_name_input_clone.is_empty() {
                    let existing_team_name = match &self.state.game_state.as_ref().unwrap().stage {
                        PublicGameStage::Teams(teams_state) => {
                            teams_state[team_index].team_name.clone()
                        }
                        // not in teams stage, do nothing
                        _ => return false,
                    };
                    self.link.send_message(match &team_option {
                        TeamOption::TeamA => AppMsg::SetTeamANameInput(existing_team_name),
                        TeamOption::TeamB => AppMsg::SetTeamBNameInput(existing_team_name),
                    });
                    return true;
                } else if validate_team_name(&team_name_input_clone).is_some() {
                    return false;
                }

                self._send_ws_message(&CTSMsg::RenameTeam {
                    team_name: team_name_input_clone,
                    team_option,
                });

                false
            }
            CTSMsgInternal::StartGrandTichu => {
                if !self.can_start_game() {
                    warn!("State is not ready to start game. Ignoring request to send websocket message.");
                    return false;
                }
                self._send_ws_message(&CTSMsg::StartGrandTichu);
                false
            }
            CTSMsgInternal::CallGrandTichu(call_grand_tichu_request) => {
                if !self.can_call_or_decline_grand_tichu() {
                    return false;
                }

                self._send_ws_message(&CTSMsg::CallGrandTichu(call_grand_tichu_request));
                true
            }
            CTSMsgInternal::CallSmallTichu => {
                self._send_ws_message(&CTSMsg::CallSmallTichu);
                false
            }
            CTSMsgInternal::SubmitTrade => {
                if !self.can_submit_trade() {
                    warn!("Invalid state to submit trade");
                    return false;
                }

                // extract user_ids from Trade state
                let (teammate_user_id, opponent1_user_id, opponent2_user_id) =
                    if let Some(game_state) = &self.state.game_state {
                        if let PublicGameStage::Trade(trade_state) = &game_state.stage {
                            let current_team = trade_state
                                .teams
                                .iter()
                                .find(|team| {
                                    team.user_ids
                                        .iter()
                                        .any(|user_id| *user_id == self.state.user_id)
                                })
                                .expect("Error finding current user's in Team state");
                            let opposing_team = trade_state
                                .teams
                                .iter()
                                .find(|team| {
                                    team.user_ids
                                        .iter()
                                        .all(|user_id| *user_id != self.state.user_id)
                                })
                                .expect("Error finding opposing team in Team state");

                            (
                                current_team
                                    .user_ids
                                    .iter()
                                    .find(|user_id| **user_id != *self.state.user_id)
                                    .expect("Error retrieving teammate's user_id from team state"),
                                opposing_team.user_ids[0].clone(),
                                opposing_team.user_ids[1].clone(),
                            )
                        } else {
                            warn!("Can't submit trade since current game stage is not Trade");
                            return false;
                        }
                    } else {
                        warn!("Can't submit trade since current game state is None");
                        return false;
                    };

                // clear selected card / trade state
                self.link.send_message_batch(vec![
                    AppMsg::RemoveSelectedPrePlayCard,
                    AppMsg::RemoveTrade(OtherPlayerOption::Opponent1),
                    AppMsg::RemoveTrade(OtherPlayerOption::Teammate),
                    AppMsg::RemoveTrade(OtherPlayerOption::Opponent2),
                ]);

                // create SubmitTrade body data
                let submit_trade = [
                    CardTrade {
                        to_user_id: opponent1_user_id,
                        card: self.state.trade_to_opponent1.as_ref().unwrap().clone(),
                        from_user_id: self.state.user_id.clone(),
                    },
                    CardTrade {
                        to_user_id: teammate_user_id.to_string(),
                        card: self.state.trade_to_teammate.as_ref().unwrap().clone(),
                        from_user_id: self.state.user_id.clone(),
                    },
                    CardTrade {
                        to_user_id: opponent2_user_id,
                        card: self.state.trade_to_opponent2.as_ref().unwrap().clone(),
                        from_user_id: self.state.user_id.clone(),
                    },
                ];

                self._send_ws_message(&CTSMsg::SubmitTrade(submit_trade));
                false
            }
            _ => {
                warn!("Tried to send unexpected message type {:?}", &msg_type);
                false
            }
        }
    }

    /// Helper function to actually send the websocket message
    fn _send_ws_message(&mut self, msg: &CTSMsg) {
        match self.ws {
            None => {
                warn!("Can't send message. Websocket is not connected.");
            }
            Some(ref mut ws_task) => {
                let msg = bincode::serialize(&msg).expect("Could not serialize message");
                ws_task.send_binary(Binary::Ok(msg));
            }
        }
    }
}
