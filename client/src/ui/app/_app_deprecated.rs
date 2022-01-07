use anyhow::Error;
use common::{
    clean_up_display_name, clean_up_game_code, get_card_combination,
    get_user_can_play_wished_for_card, next_combo_beats_prev, sort_cards_for_hand,
    validate_display_name, validate_game_code, validate_team_name, CTSMsg, CallGrandTichuRequest,
    Card, CardTrade, CardValue, Deck, ImmutableTeam, MutableTeam, OtherPlayerOption,
    PublicGameStage, PublicGameState, PublicUser, STCMsg, TeamCategories, TeamOption,
    TichuCallStatus, ValidCardCombo, DRAGON, MAH_JONG, NO_USER_ID,
};
use gloo::{
    storage::{LocalStorage, Storage},
    timers::callback::Interval,
};
use log::*;
use serde_derive::{Deserialize, Serialize};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{EventTarget, HtmlInputElement, HtmlSelectElement, MessageEvent, WebSocket};
use yew::{html::Scope, prelude::*};

use crate::global::ws::CTSMsgInternal;

type ShouldRender = bool;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum WSConnectionStatus {
    Open,
    Error,
    Closed,
}

pub struct App {
    state: State,
    ws: Option<WebSocket>,
    ws_callbacks: Option<WSCallbacks>,
    ping_interval: Option<Interval>,
}

/// Stores callbacks that are passed to javascript so that they don't get dropped
/// and so we don't have to `.forget()` them when passing them to JS.
struct WSCallbacks {
    #[allow(dead_code)]
    onmessage: Closure<dyn FnMut(MessageEvent)>,
    #[allow(dead_code)]
    onopen: Closure<dyn FnMut()>,
    #[allow(dead_code)]
    onerror: Closure<dyn FnMut(ErrorEvent)>,
    #[allow(dead_code)]
    onclose: Closure<dyn FnMut()>,
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

    /// temporary card selected for trading, etc. (i.e. game stage is not Play)
    /// this card is then moved into `trade_to_opponent` state once the user chooses
    /// who to give the card to
    selected_pre_play_card: Option<Card>,
    trade_to_opponent1: Option<Card>,
    trade_to_teammate: Option<Card>,
    trade_to_opponent2: Option<Card>,

    /// cards selected for playing
    selected_play_cards: Vec<Card>,
    wished_for_card_value: Option<CardValue>,
    user_id_to_give_dragon_to: Option<String>,
    show_user_id_to_give_dragon_to_form: bool,
}

const USER_ID_STORAGE_KEY: &str = "yew.tichu.user_id";
const DISPLAY_NAME_STORAGE_KEY: &str = "yew.tichu.display_name";

#[derive(Clone, Debug, PartialEq)]
struct Theme {
    foreground: String,
    background: String,
}

pub enum AppMsg {
    ConnectToWS,
    BeginPing,
    WebsocketOpen,
    WebsocketError,
    WebsocketClosed,
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
    SetUserIdToGiveDragonTo(Option<String>),
    SetWishedForCard(usize),
}

const PING_INTERVAL_MS: u32 = 5000;

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_context: &yew::Context<Self>) -> Self {
        // retrieve user_id and display name from local storage
        let user_id =
            LocalStorage::get(USER_ID_STORAGE_KEY).unwrap_or_else(|_| String::from(NO_USER_ID));
        let display_name =
            LocalStorage::get(DISPLAY_NAME_STORAGE_KEY).unwrap_or_else(|_| String::from(""));

        // store user_id and display_name in local storage (if changed)
        LocalStorage::set(USER_ID_STORAGE_KEY, &user_id)
            .expect("failed to save user_id to local storage");
        LocalStorage::set(DISPLAY_NAME_STORAGE_KEY, &display_name)
            .expect("failed to save display_name to local storage");

        let state = State {
            ws_connection_status: WSConnectionStatus::Closed,
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
            user_id_to_give_dragon_to: None,
            show_user_id_to_give_dragon_to_form: false,
            wished_for_card_value: None,
        };
        Self {
            ws: None,
            ws_callbacks: None,
            ping_interval: None,
            state,
        }
    }

    fn rendered(&mut self, context: &yew::Context<Self>, first_render: bool) {
        let link = context.link();
        // connect to websocket on first render
        if self.ws.is_none() && first_render {
            link.send_message(AppMsg::ConnectToWS);
            link.send_message(AppMsg::BeginPing);
        }
    }

    fn update(&mut self, context: &yew::Context<Self>, msg: Self::Message) -> ShouldRender {
        let link = context.link();
        match msg {
            AppMsg::WebsocketOpen => {
                self.state.is_alive = true;
                self.state.ws_connection_status = WSConnectionStatus::Open;
                true
            }
            AppMsg::WebsocketError => {
                self.state.is_alive = false;
                self.state.ws_connection_status = WSConnectionStatus::Error;
                true
            }
            AppMsg::WebsocketClosed => {
                self.state.is_alive = false;
                self.state.ws_connection_status = WSConnectionStatus::Closed;
                true
            }
            AppMsg::ConnectToWS => {
                if self.ws.is_none() {
                    info!("Connecting to websocket...");
                    let url = format!("ws://localhost:8080/ws?user_id={}", self.state.user_id);
                    let ws = WebSocket::new(&url).expect("Should connect to URL without error");
                    ws.set_binary_type(web_sys::BinaryType::Arraybuffer);
                    let onmessage_link = link.clone();
                    let onopen_link = link.clone();
                    let onerror_link = link.clone();
                    let onclose_link = link.clone();
                    // on message
                    let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
                        // ArrayBuffer
                        if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
                            let u_int_8_array = js_sys::Uint8Array::new(&abuf);
                            let vec = u_int_8_array.to_vec();
                            onmessage_link.send_message(AppMsg::WSMsgReceived(Ok(vec)));
                        } else if let Ok(blob) = e.data().dyn_into::<web_sys::Blob>() {
                            // Blob
                            warn!("Websocket message event, received blob: {:?}", blob);
                        } else if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                            // Text
                            warn!("Websocket message event, received Text: {:?}", txt);
                        } else {
                            // Unknown
                            warn!("Websocket message event, received Unknown: {:?}", e.data());
                        }
                    })
                        as Box<dyn FnMut(MessageEvent)>);
                    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));

                    // on open
                    let onopen_callback = Closure::wrap(Box::new(move || {
                        info!("Websocket open event");
                        onopen_link.send_message(AppMsg::WebsocketOpen);
                    }) as Box<dyn FnMut()>);
                    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));

                    // on error
                    let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
                        error!("Websocket event: {:?}", e);
                        onerror_link.send_message(AppMsg::WebsocketError);
                    })
                        as Box<dyn FnMut(ErrorEvent)>);
                    ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));

                    // on close
                    let onclose_callback = Closure::wrap(Box::new(move || {
                        error!("Websocket close event");
                        onclose_link.send_message(AppMsg::WebsocketClosed);
                    })
                        as Box<dyn FnMut()>);
                    ws.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));

                    self.ws = Some(ws);
                    self.ws_callbacks = Some(WSCallbacks {
                        onmessage: onmessage_callback,
                        onopen: onopen_callback,
                        onerror: onerror_callback,
                        onclose: onclose_callback,
                    });
                } else {
                    error!("Trying to ConnectToWS while current websocket is still defined as Some() in state");
                }
                true
            }
            AppMsg::BeginPing => {
                let link = link.clone();
                let interval = Interval::new(PING_INTERVAL_MS, move || {
                    link.send_message(AppMsg::SendWSMsg(CTSMsgInternal::Ping));
                });
                self.ping_interval = Some(interval);
                false
            }
            AppMsg::SendWSMsg(msg_type) => self.send_ws_message(link, msg_type),
            AppMsg::WSMsgReceived(data) => self.handle_ws_message_received(link, data),
            AppMsg::SetUserId(s) => {
                LocalStorage::set(USER_ID_STORAGE_KEY, &s)
                    .expect("failed to save user_id to local storage");
                self.state.user_id = s;

                false
            }
            AppMsg::SetDisplayName(s) => {
                let s = clean_up_display_name(&s);
                LocalStorage::set(DISPLAY_NAME_STORAGE_KEY, &s)
                    .expect("failed to save display_name to local storage");
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
            AppMsg::SetUserIdToGiveDragonTo(user_id) => {
                self.state.user_id_to_give_dragon_to = user_id;
                true
            }
            AppMsg::SetWishedForCard(i) => {
                let wished_for_card_value = Deck::get_wished_for_card_value_from_i(i);
                self.state.wished_for_card_value = wished_for_card_value;
                true
            }
        }
    }

    fn changed(&mut self, _prop: &yew::Context<App>) -> bool {
        false
    }

    fn destroy(&mut self, _context: &yew::Context<Self>) {}

    fn view(&self, context: &yew::Context<Self>) -> Html {
        let link = context.link();
        html! {
                <div>
                    {match &self.state.game_state {
                        None => self.view_join(context.link()),
                        Some(game_state) =>{
                            match game_state.stage {
                                PublicGameStage::Lobby => self.view_lobby(link),
                                PublicGameStage::Teams(_) => self.view_teams(link),
                                PublicGameStage::GrandTichu(_) => self.view_grand_tichu(link),
                                PublicGameStage::Trade(_) => self.view_trade(link),
                                PublicGameStage::Play(_) => self.view_play(link),
                                _ => html!{<> </>}
                        }
                    }
                }}
                    <br />
                    <br />
                    <hr />
                    <br />
                    {self.view_debug(link)}
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
                PublicGameStage::Teams(teams) => teams[0].user_ids.contains(&self.state.user_id),
                _ => false,
            },
        }
    }

    fn is_on_team_b(&self) -> bool {
        match &self.state.game_state {
            None => false,
            Some(game_state) => match &game_state.stage {
                PublicGameStage::Teams(teams) => teams[1].user_ids.contains(&self.state.user_id),
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

    fn debug_mutable_team(&self, team: &MutableTeam) -> Html {
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

    fn debug_immutable_team(&self, team: &ImmutableTeam) -> Html {
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
                            <li>{self.debug_mutable_team(&team_state[0])}</li>
                            <li>{self.debug_mutable_team(&team_state[1])}</li>
                        </ul>
                    }
                }
                PublicGameStage::GrandTichu(grand_tichu_state) => {
                    html! {
                        <ul>
                            <li>{self.debug_immutable_team(&grand_tichu_state.teams[0])}</li>
                            <li>{self.debug_immutable_team(&grand_tichu_state.teams[1])}</li>
                        </ul>
                    }
                }
                PublicGameStage::Trade(trade_state) => {
                    html! {
                        <ul>
                            <li>{self.debug_immutable_team(&trade_state.teams[0])}</li>
                            <li>{self.debug_immutable_team(&trade_state.teams[1])}</li>
                        </ul>
                    }
                }
                PublicGameStage::Play(play_state) => {
                    html! {
                        <ul>
                            <li>{self.debug_immutable_team(&play_state.teams[0])}</li>
                            <li>{self.debug_immutable_team(&play_state.teams[1])}</li>
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
                    html! {}
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

    fn view_debug(&self, link: &Scope<App>) -> Html {
        html! {
                <>
                <h1>{"Debug Info:"}</h1>
                    <p>{"Display Name: "} {&self.state.display_name}</p>
                    <p>{"User ID: "} {&self.state.user_id}</p>
                    <p>{"Websocket Status: "}{match &self.state.ws_connection_status {
                        WSConnectionStatus::Open => "Connected",
                        WSConnectionStatus::Closed => "Not Connected",
                        WSConnectionStatus::Error => "Error"
                }}</p>
                    <p>{"Game Code: "} {
                        if let Some(game_state) = &self.state.game_state {
                            &game_state.game_code
                    } else {
                            ""
                    }}
                    </p>
                    {self.view_debug_skip_to_play(link)}
                    <p>{"Participants: "} {self.view_participants()}</p>
                    <p>{"Owner: "} {self.debug_owner()}</p>
                    <p>{"Teams: "} {self.debug_teams()}</p>
                    {self.debug_user_id_to_give_dragon_to()}
                    {self.debug_wished_for_card()}
                    <h2>{"Grand Tichus: "}</h2>
                    {self.view_debug_all_participants_grand_tichu()}
                    <br />
                    <h2>{"Small Tichus: "}</h2>
                    {self.view_debug_all_participants_small_tichu()}
                    <button onclick={link.callback(|_| AppMsg::SendWSMsg(CTSMsgInternal::Test))}>{"Send test message to server"}</button>
                    <br />
                    <button onclick={link.callback(|_| AppMsg::SendWSMsg(CTSMsgInternal::Ping))}>{"Send ping to server"}</button>
                </>
        }
    }

    fn view_join(&self, link: &Scope<Self>) -> Html {
        html! {
                <>
                <h1>{"Tichu"}</h1>
                    <form onsubmit={link.callback(|e: FocusEvent| {
                        e.prevent_default();
                        AppMsg::SendWSMsg(CTSMsgInternal::JoinGameWithGameCode)
                })}>
                        <label for="join-room-display-name-input">{"Display Name"}</label>
                        <br />
                        <input
                            id="join-room-display-name-input"
                            type="text"
                            value={self.state.display_name_input.clone()}
                            oninput={link.batch_callback(|e: InputEvent| {
                                let target: Option<EventTarget> = e.target();
                                let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
                                input.map(|input|  AppMsg::SetDisplayNameInput(input.value()))
                            })} />
                        <br />
                        <label for="join-room-game-code-input">{"Game Code"}</label>
                        <br />
                        <input
                            id="join-room-game-code-input"
                            type="text"
                            value={self.state.join_room_game_code_input.clone()}
                            oninput={link.batch_callback(|e: InputEvent| {
                                let target: Option<EventTarget> = e.target();
                                let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
                                input.map(|input| AppMsg::SetJoinRoomGameCodeInput(input.value()))
                            }) }/>
                        <br />
                        <button
                            type="submit"
                            disabled={!self.can_join_game()}
                            >{"Join game"}</button>
                    </form>
                    <br />
                    <br />
                    <form onsubmit={link.callback(|e: FocusEvent| {
                        e.prevent_default();
                        AppMsg::SendWSMsg(CTSMsgInternal::CreateGame)
                })}>
                        <label for="join-room-display-name-input">{"Display Name"}</label>
                        <br />
                        <input
                            id="create-room-display-name-input"
                            type="text"
                            value={self.state.display_name_input.clone()}
                            oninput={link.batch_callback(|e: InputEvent| {
                                let target: Option<EventTarget> = e.target();
                                let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
                                input.map(|input| AppMsg::SetDisplayNameInput(input.value()))
                            })} />
                        <br />
                        <button
                            type="submit"
                            disabled={!self.can_create_game()}
                            >{"Create game"}</button>
                    </form>
                </>
        }
    }

    fn view_lobby(&self, link: &Scope<Self>) -> Html {
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
                        onclick={link.callback(|_| {AppMsg::SendWSMsg(CTSMsgInternal::LeaveGame)})}
                        disabled={!self.can_leave_game()}
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

    fn view_teams(&self, link: &Scope<Self>) -> Html {
        html! {
                <>
                <h1>{"Teams"}</h1>
                    <label for="team-a-name-input">{"Team Name"}</label>
                    <br />
                    <form onsubmit={
                        link.callback(|e: FocusEvent | {
                        e.prevent_default();
                        AppMsg::SendWSMsg(CTSMsgInternal::RenameTeam(TeamOption::TeamA))
                    })}>
                    <label for="team-a-name-input">{"Team Name"}</label>
                    <input
                        id="team-a-name-input"
                        disabled={!self.is_team_stage() || self.is_on_team_b()}
                        type="text"
                        value={self.state.team_a_name_input.clone()}
                        oninput={link.batch_callback(|e: InputEvent| {
                            let target = e.target();
                            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
                            input.map(|input|   AppMsg::SetTeamANameInput(input.value()))
                        })} />
                   </form>
                    <br />
                    <button
                        onclick={link.callback(|_| {AppMsg::SendWSMsg(CTSMsgInternal::MoveToTeam(TeamOption::TeamA))})}
                        disabled={!self.is_team_stage() || self.is_on_team_a()}
                        >{"Move to Team A"}</button>
                    <br />
                    <button
                        onclick={link.callback(|_| {AppMsg::SendWSMsg(CTSMsgInternal::MoveToTeam(TeamOption::TeamB))})}
                        disabled={!self.is_team_stage() || self.is_on_team_b()}
                        >{"Move to Team B"}</button>
                    <br />
                    <br />
                    <form onsubmit={link.callback(|e: FocusEvent | {
                        e.prevent_default();
                        AppMsg::SendWSMsg(CTSMsgInternal::RenameTeam(TeamOption::TeamB))
                        })}>
                        <label for="team-b-name-input">{"Team Name"}</label>
                        <input
                            id="team-b-name-input"
                            disabled={!self.is_team_stage() || self.is_on_team_a()}
                            type="text"
                            value={self.state.team_b_name_input.clone()}
                            oninput={link.batch_callback(|e: InputEvent| {
                                let target = e.target();
                                let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
                                input.map(|input|  AppMsg::SetTeamBNameInput(input.value()))
                            })} />
                   </form>
                   {if self.is_current_user_owner() {
                      html!{
                        <button
                            onclick={link.callback(|_| {AppMsg::SendWSMsg(CTSMsgInternal::StartGrandTichu)})}
                            disabled={!self.can_start_game()}
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

    fn view_pre_play_hand(&self, link: &Scope<Self>) -> Html {
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
                                            disabled={!self.can_select_pre_play_card()}
                                            onclick={link.callback(move |_| {AppMsg::SetSelectedPrePlayCard(i)})}
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

    fn call_small_tichu_button(&self, link: &Scope<Self>) -> Html {
        html! {
                <button
                onclick={link.callback(|_| {AppMsg::SendWSMsg(CTSMsgInternal::CallSmallTichu)})}
                disabled={!self.can_call_small_tichu()}
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

    fn view_grand_tichu(&self, link: &Scope<Self>) -> Html {
        html! {
                <>
                    <h1>{"Grand Tichu"}</h1>
                    {self.view_grand_tichu_status_for_current_user()}
                    <button
                        onclick={link.callback(|_| {AppMsg::SendWSMsg(CTSMsgInternal::CallGrandTichu(CallGrandTichuRequest::Call))})}
                        disabled={!self.can_call_or_decline_grand_tichu()}
                    >{"Call Grand Tichu"}</button>
                    <button
                        onclick={link.callback(|_| {AppMsg::SendWSMsg(CTSMsgInternal::CallGrandTichu(CallGrandTichuRequest::Decline))})}
                        disabled={!self.can_call_or_decline_grand_tichu()}
                    >{"Decline Grand Tichu"}</button>
                    {self.call_small_tichu_button(link)}
                    <p>{"Hand:"}</p>
                    {self.view_pre_play_hand(link)}
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

    fn view_selected_pre_play_card_button(&self, link: &Scope<Self>) -> Html {
        match &self.state.selected_pre_play_card {
            Some(card) => {
                html! {
                        <button
                            onclick={link.callback(|_| {AppMsg::RemoveSelectedPrePlayCard})}
                            disabled={self.state.selected_pre_play_card.is_none()}
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

    fn view_trade_to_person(&self, link: &Scope<Self>, trade_to_person: OtherPlayerOption) -> Html {
        let trade_to_person_state = match &trade_to_person {
            OtherPlayerOption::Opponent1 => &self.state.trade_to_opponent1,
            OtherPlayerOption::Teammate => &self.state.trade_to_teammate,
            OtherPlayerOption::Opponent2 => &self.state.trade_to_opponent2,
        };

        let trade_to_person_display_name = {
            let public_other_players = &self.state.game_state.as_ref().unwrap().get_other_players();

            if let Some(public_other_players) = public_other_players {
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
            }
        };

        return html! {
              <>
                {if self.state.selected_pre_play_card.is_none() {
                    html!{
                        <button
                            disabled={!self.can_remove_trade(&trade_to_person)}
                            onclick={link.callback(move |_| {AppMsg::RemoveTrade(trade_to_person.clone())})}
                        >
                        {match trade_to_person_state {
                            Some(card) => format!("Remove {:?} to {}", card, trade_to_person_display_name),
                            None => format!("No trade selected for {}", trade_to_person_display_name),
                        }}
                        </button>
                    }
                } else {
                        html!{
                            <button onclick={link.callback(move |_| {AppMsg::SetTrade(trade_to_person.clone())})}>
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
                trade_state.submitted_trades.contains(&self.state.user_id)
            } else {
                false
            }
        } else {
            false
        }
    }

    fn view_trade(&self, link: &Scope<Self>) -> Html {
        html! {
                <>
                    <h1>{"Trade"}</h1>
                    <p>{&format!("Has submitted trade: {:?}", self.has_submitted_trade())}</p>
                    {if !self.has_submitted_trade() {
                        html!{
                            <>
                                <button
                                    onclick={link.callback(|_| {AppMsg::SendWSMsg(CTSMsgInternal::SubmitTrade)})}
                                    disabled={!self.can_submit_trade()}
                                    type="submit">
                                {"Submit"}
                                </button>
                                <br />
                                <br />
                                {self.view_trade_to_person(link, OtherPlayerOption::Opponent1)}
                                <br />
                                {self.view_trade_to_person(link, OtherPlayerOption::Teammate)}
                                <br />
                                {self.view_trade_to_person(link, OtherPlayerOption::Opponent2)}
                                <br />
                                <br />
                                <br />
                                {self.call_small_tichu_button(link)}
                                <br />
                                <br />
                                <br />
                                {self.view_selected_pre_play_card_button(link)}
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
                    {self.view_pre_play_hand(link)}
                </>
        }
    }

    fn is_play_card_selected(&self, card: &Card) -> bool {
        self.state.selected_play_cards.contains(card)
    }

    fn can_select_play_card(&self) -> bool {
        self.stage_is_play()
    }

    fn hand_beats_combo_on_table(&self, next_combo: &ValidCardCombo) -> bool {
        let prev_combo = if let Some(game_state) = &self.state.game_state {
            if let PublicGameStage::Play(play_stage) = &game_state.stage {
                play_stage.table.last()
            } else {
                // game stage is not Play
                return false;
            }
        } else {
            // game state is not instantiated
            return false;
        };
        next_combo_beats_prev(&prev_combo, next_combo)
    }

    fn is_current_users_turn(&self) -> bool {
        if let Some(game_state) = &self.state.game_state {
            if let PublicGameStage::Play(play_state) = &game_state.stage {
                return play_state.turn_user_id == self.state.user_id;
            }
        }
        false
    }

    fn get_prev_played_combo(&self) -> Option<&ValidCardCombo> {
        return if let Some(game_state) = &self.state.game_state {
            if let PublicGameStage::Play(play_stage) = &game_state.stage {
                play_stage.table.last()
            } else {
                None
            }
        } else {
            None
        };
    }

    fn view_is_valid_combo(&self) -> Html {
        html!(
            <p>
            {if get_card_combination(self.get_prev_played_combo(), &self.state.selected_play_cards, &self.state.user_id).is_some() {
                "Valid combination"
            } else {
                "Invalid combination"
            }}
            </p>
        )
    }

    fn can_play_cards(&self) -> bool {
        let game_state = if let Some(game_state) = &self.state.game_state {
            game_state
        } else {
            return false;
        };

        // must be users turn OR must be playable bomb
        let combo = get_card_combination(
            self.get_prev_played_combo(),
            &self.state.selected_play_cards,
            &self.state.user_id,
        );

        let combo_contains_dragon = self.state.selected_play_cards.contains(&DRAGON);
        let user_has_chosen_a_user_to_given_dragon_to =
            self.state.user_id_to_give_dragon_to.is_some();

        let wished_for_card_value = game_state.get_wished_for_card_value();

        let some_card_has_been_wished_for = wished_for_card_value.is_some();

        if let Some(combo) = combo {
            let user_can_play_wished_for_card = if some_card_has_been_wished_for {
                let wished_for_card_value = wished_for_card_value.as_ref().unwrap();
                get_user_can_play_wished_for_card(
                    self.get_prev_played_combo(),
                    &game_state.current_user.hand,
                    wished_for_card_value,
                )
            } else {
                false
            };
            let combo_contains_wished_for_card = if some_card_has_been_wished_for {
                let wished_for_card_value = wished_for_card_value.as_ref().unwrap();
                combo
                    .cards()
                    .iter()
                    .any(|card| card.value == *wished_for_card_value)
            } else {
                false
            };

            self.stage_is_play()
                && (self.is_current_users_turn() || combo.is_bomb())
                && self.hand_beats_combo_on_table(&combo)
                && (!combo_contains_dragon
                    || combo_contains_dragon && user_has_chosen_a_user_to_given_dragon_to)
                && (!some_card_has_been_wished_for
                    || (some_card_has_been_wished_for && !user_can_play_wished_for_card)
                    || (some_card_has_been_wished_for
                        && user_can_play_wished_for_card
                        && combo_contains_wished_for_card))
        } else {
            // cards are not a valid combo
            false
        }
    }

    fn view_play_hand(&self, link: &Scope<Self>) -> Html {
        if let Some(game_state) = &self.state.game_state {
            html! {
                    <ul>
                        {for game_state.current_user.hand.iter().enumerate().map(|(i, card)| {
                            // do not render card if the stage is Trade and it is currently selected
                            // OR if it has been selected for trade with opponent
                        let card_is_selected = self.state.selected_play_cards.contains(card);
                        if card_is_selected {
                            html!{}
                        } else {
                                html!{
                                    <li>
                                        <button
                                            disabled={!self.can_select_play_card()}
                                            onclick={link.callback(move |_| {AppMsg::AddSelectedPlayCard(i)})}
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

    fn view_selected_play_card_buttons(&self, link: &Scope<App>) -> Html {
        html! {
            <>
            { for self.state.selected_play_cards.iter().enumerate().map(|(i, selected_card)| {
                 html! {
                     <button
                         onclick={link.callback(move |_| {AppMsg::RemoveSelectedPlayCard(i)})}
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
                let turn_user = &game_state.get_user_by_user_id(turn_user_id);
                if let Some(turn_user) = turn_user {
                    return html! {
                        <p>{&format!("Current turn: {}", turn_user.display_name)}</p>
                    };
                }
            }
        }

        html! {}
    }

    fn view_cards_on_table(&self) -> Html {
        if let Some(game_state) = &self.state.game_state {
            if let PublicGameStage::Play(play_state) = &game_state.stage {
                let last_combo = play_state.table.last();
                if let Some(last_combo) = last_combo {
                    html! {
                            <>
                                <p>{"Cards on table:"}</p>
                                <ul>
                                    {for last_combo.cards().iter().map(|card| {
                                        html!{
                                            <li>
                                                <p>
                                                    {&format!("{:#?}", card)}
                                                </p>
                                            </li>
                                    }
                                })}
                                </ul>
                           </>
                    }
                } else {
                    html! {  <p>{"No cards on table yet"}</p> }
                }
            } else {
                html! {}
            }
        } else {
            html! {}
        }
    }

    fn debug_wished_for_card(&self) -> Html {
        let wished_for_card_value = if let Some(game_state) = &self.state.game_state {
            game_state.get_wished_for_card_value()
        } else {
            None
        };

        html! {
            <>
              <p>{"Wished for Card:"}</p>
              <p>{format!("{:#?}", wished_for_card_value)}</p>
            </>
        }
    }

    fn view_debug_skip_to_play(&self, link: &Scope<Self>) -> Html {
        html! {
            <button
                onclick={link.callback(move |_| AppMsg::SendWSMsg(CTSMsgInternal::__Admin_SkipToPlay))}
            >
                {"Skip to Play Stage"}
            </button>
        }
    }

    fn view_wish_for_card_input(&self, link: &Scope<Self>) -> Html {
        html! {
            <>
                <label for="wish-for-card">{"Wish for a card?"}</label>
                <select name="wish-for-card" id="wish-for-card"
                    oninput={link.batch_callback(move |e: InputEvent| {
                        let target = e.target();
                        let select = target.and_then(|t| t.dyn_into::<HtmlSelectElement>().ok());
                        select.map(|select|  AppMsg::SetWishedForCard(select.selected_index() as usize))
                    })}
                >
                    {for Deck::wished_for_card_values().iter().enumerate().map(|(i, card)| {
                        let card_string = format!("{:#?}", card);
                        html!{
                            <option value={format!("{}", i)}>
                                {card_string.clone()}
                            </option>
                        }
                    })}
                </select>
            </>
        }
    }

    fn debug_user_id_to_give_dragon_to(&self) -> Html {
        let user_id_to_give_dragon_to = &self.state.user_id_to_give_dragon_to;
        html! {
            <>
              <p>{"User To Give Dragon To:"}</p>
              <p>{format!("{:#?}", &user_id_to_give_dragon_to)}</p>
              {if let Some(game_state) = &self.state.game_state {
                if let Some(user_id_to_give_dragon_to) = user_id_to_give_dragon_to {
                    html!{
                        <p>{format!("{:#?}", game_state.get_user_by_user_id(user_id_to_give_dragon_to).unwrap().display_name)}</p>
                    }
                } else {
                    html!{}
                }
              } else {
                  html!{}
              }}
            </>
        }
    }

    fn get_opponent_ids(&self) -> Option<(String, String)> {
        if let Some(game_state) = &self.state.game_state {
            if let Some(TeamCategories { opposing_team, .. }) =
                game_state.get_immutable_team_categories()
            {
                Some((
                    opposing_team.user_ids[0].clone(),
                    opposing_team.user_ids[1].clone(),
                ))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get_user_must_select_user_id_to_give_dragon_to(&self) -> bool {
        let opponent_ids = self.get_opponent_ids();
        if let Some(game_state) = &self.state.game_state {
            // game stage is Play
            if let PublicGameStage::Play(play_state) = &game_state.stage {
                // this is the last user to pass

                if play_state.passes.iter().filter(|pass| pass.passed).count() == 3 {
                    // last trick has a dragon in it
                    if play_state
                        .table
                        .last()
                        .expect("There should be cards on the table")
                        .cards()
                        .contains(&DRAGON)
                    {
                        let (opponent_id_0, opponent_id_1) =
                            opponent_ids.expect("Opponents should be found in state");
                        // user has chosen SOME user to give the dragon to
                        return if let Some(user_id_to_give_dragon_to) =
                            &self.state.user_id_to_give_dragon_to
                        {
                            // user has chosen an opponent to give the dragon to
                            !(user_id_to_give_dragon_to == &opponent_id_0
                                || user_id_to_give_dragon_to == &opponent_id_1)
                        } else {
                            // user has NOT chosen an opponent to give the dragon to
                            true
                        };
                    }
                }
            }
        }

        false
    }

    fn get_user_has_selected_user_id_to_give_dragon_to(&self) -> bool {
        let opponent_ids = self.get_opponent_ids();
        if let Some(game_state) = &self.state.game_state {
            // game stage is Play
            if let PublicGameStage::Play(play_state) = &game_state.stage {
                // this is the last user to pass

                if play_state.passes.iter().filter(|pass| pass.passed).count() == 3 {
                    // last trick has a dragon in it
                    if play_state
                        .table
                        .iter()
                        .any(|combo| combo.cards().contains(&DRAGON))
                    {
                        let (opponent_id_0, opponent_id_1) =
                            opponent_ids.expect("Opponents should be found in state");
                        // user has chosen an opponent to give the dragon to
                        return if let Some(user_id_to_give_dragon_to) =
                            &self.state.user_id_to_give_dragon_to
                        {
                            user_id_to_give_dragon_to == &opponent_id_0
                                || user_id_to_give_dragon_to == &opponent_id_1
                        } else {
                            false
                        };
                    }
                }
            }
        }

        false
    }

    fn get_can_pass(&self) -> bool {
        if let Some(game_state) = &self.state.game_state {
            // game stage is Play
            if let PublicGameStage::Play(play_state) = &game_state.stage {
                // it is the users turn
                if play_state.turn_user_id == self.state.user_id {
                    // user doesn't have to choose an opponent OR does have to choose an opponent and has done so
                    return !self.get_user_must_select_user_id_to_give_dragon_to()
                        || self.get_user_has_selected_user_id_to_give_dragon_to();
                }
            }
        }

        false
    }

    fn view_choose_opponent(&self, link: &Scope<Self>) -> Html {
        let opponent_ids = self.get_opponent_ids();

        if let Some(game_state) = &self.state.game_state {
            if let Some((opponent_id_0, opponent_id_1)) = opponent_ids {
                let opponent_id_0_clone = opponent_id_0.clone();
                let opponent_id_1_clone = opponent_id_1.clone();
                html! {
                    <>
                        <p>{"Choose opponent"}</p>
                        // opponent 0
                        {if self.state.user_id_to_give_dragon_to == Some(opponent_id_0_clone.clone()) {
                            html! {
                                <button
                                    onclick={link.callback(move |_| AppMsg::SetUserIdToGiveDragonTo(None))}
                                >
                                    {format!("Deselect {}", &game_state.get_user_by_user_id(&opponent_id_0_clone).unwrap().display_name)}
                                </button>
                            }
                        } else {
                            html!{
                                <button
                                    onclick={link.callback(move |_| AppMsg::SetUserIdToGiveDragonTo(Some(opponent_id_0.clone())))}
                                >
                                    {format!("Select {}", &game_state.get_user_by_user_id(&opponent_id_0_clone).unwrap().display_name)}
                                </button>
                            }
                        }}
                        // opponent 1
                        {if self.state.user_id_to_give_dragon_to == Some(opponent_id_1_clone.clone()) {
                            html! {
                                <button
                                    onclick={link.callback(move |_| AppMsg::SetUserIdToGiveDragonTo(None))}
                                >
                                    {format!("Deselect {}", &game_state.get_user_by_user_id(&opponent_id_1_clone).unwrap().display_name)}
                                </button>
                            }
                        } else {
                            html!{
                                <button
                                    onclick={link.callback(move |_| AppMsg::SetUserIdToGiveDragonTo(Some(opponent_id_1.clone())))}
                                >
                                    {format!("Select {}", &game_state.get_user_by_user_id(&opponent_id_1_clone).unwrap().display_name)}
                                </button>
                            }
                        }}
                    </>
                }
            } else {
                html! {}
            }
        } else {
            html! {}
        }
    }

    fn view_play(&self, link: &Scope<Self>) -> Html {
        html! {
              <>
                <h1>{"Play"}</h1>
                <br />
                <br />
                {self.view_turn_display_name()}
                <br />
                <br />
                {self.view_cards_on_table()}
                <br />
                <br />
                {if self.state.selected_play_cards.contains(&MAH_JONG) {
                    self.view_wish_for_card_input(link)
                } else {
                    html!{}
                }}
                <br />
                <br />
                <p>{"Real self.view_choose_opponent:"}</p>
                {if self.state.selected_play_cards.contains(&DRAGON) {
                    self.view_choose_opponent(link)
                } else {
                    html!{}
                }}
                <br />
                <br />
                <button
                    disabled={!self.can_play_cards()}
                    onclick={link.callback(|_| AppMsg::SendWSMsg(CTSMsgInternal::PlayCards))}
                    type="submit"
                    >
                    {"Submit cards"}
                </button>
                <br />
                <br />
                {self.call_small_tichu_button(link)}
                <br />
                <br />
                {self.view_is_valid_combo()}
                <br />
                <br />
                {self.view_selected_play_card_buttons(link)}
                <br />
                <br />
                {self.view_play_hand(link)}
              </>
        }
    }

    /// Handles when a websocket message is received from the server
    /// Returns whether the component should re-render or not
    fn handle_ws_message_received(
        &mut self,
        link: &Scope<Self>,
        data: Result<Vec<u8>, Error>,
    ) -> bool {
        let mut should_rerender = true;
        if data.is_err() {
            error!("Data received from websocket was an error {:#?}", &data);
            return false;
        }
        let data: Option<STCMsg> = bincode::deserialize(&data.unwrap()).ok();
        info!("Received websocket message: {:#?}", &data);
        match data {
            None => {
                warn!("Deserialized data is None. This probably indicates there was an error deserializing the websocket message binary");
            }
            Some(data) => match data {
                STCMsg::Ping => {
                    link.send_message(AppMsg::SendWSMsg(CTSMsgInternal::Pong));
                }
                STCMsg::UserIdAssigned(s) => {
                    link.send_message(AppMsg::SetUserId(s));
                }
                STCMsg::GameState(new_game_state) => {
                    let new_game_state = *new_game_state;

                    // if team names are empty, update team name inputs to reflect state
                    if self.state.team_a_name_input.is_empty()
                        || self.state.team_b_name_input.is_empty()
                    {
                        if let Some(new_game_state) = &new_game_state {
                            if let PublicGameStage::Teams(teams_state) = &new_game_state.stage {
                                link.send_message_batch(vec![
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
                        link.send_message(AppMsg::SetDisplayName(
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
                    link.send_message(AppMsg::SetTeamANameInput(new_team_a_name));
                }
                STCMsg::TeamBRenamed(new_team_b_name) => {
                    link.send_message(AppMsg::SetTeamBNameInput(new_team_b_name));
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
                STCMsg::CardsPlayed => {}
                STCMsg::FirstCardsDealt => {}
                STCMsg::LastCardsDealt => {}
                STCMsg::PlayerReceivedDragon => {}
                STCMsg::GameEnded => {}
                STCMsg::GameEndedFinal => {}
                STCMsg::UserPassed(_) => {}
            },
        }

        should_rerender
    }

    /// Sends a message to the server via websocket
    /// Returns whether the component should rerender
    fn send_ws_message(&mut self, link: &Scope<Self>, msg_type: CTSMsgInternal) -> bool {
        info!("Sending websocket message: {:#?}", msg_type);
        match msg_type {
            CTSMsgInternal::Test => {
                self._send_ws_message(&CTSMsg::Test(String::from("Hello, server!")));
                false
            }
            CTSMsgInternal::Ping => {
                let should_reconnect = if self.ws.is_none() {
                    info!("Trying to ping, but there is no websocket connection. Attempting to reconnect");
                    true
                } else if !self.state.is_alive {
                    info!("Trying to ping, but websocket is not alive. Closing websocket connection and attempting to reconnect.");
                    drop(self.ws.take());
                    drop(self.ws_callbacks.take());
                    true
                } else {
                    false
                };

                self.state.is_alive = false;
                if should_reconnect {
                    link.send_message(AppMsg::ConnectToWS);
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
                    link.send_message(match &team_option {
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
                                .find(|team| team.user_ids.contains(&self.state.user_id))
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
                link.send_message_batch(vec![
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
            CTSMsgInternal::PlayCards => {
                if !self.can_play_cards() {
                    return false;
                }

                let cards = self.state.selected_play_cards.clone();
                let wished_for_card_value = self.state.wished_for_card_value.clone();
                let user_id_to_give_dragon_to = self.state.user_id_to_give_dragon_to.clone();

                // reset state
                self.state.selected_play_cards.drain(..);
                self.state.user_id_to_give_dragon_to = None;
                self.state.wished_for_card_value = None;

                self._send_ws_message(&CTSMsg::PlayCards {
                    cards,
                    wished_for_card_value,
                    user_id_to_give_dragon_to,
                });
                false
            }
            CTSMsgInternal::Pass => {
                if !self.get_can_pass() {
                    // user must select an opponent to give the dragon to
                    if self.get_user_must_select_user_id_to_give_dragon_to()
                        && !self.get_user_has_selected_user_id_to_give_dragon_to()
                    {
                        self.state.show_user_id_to_give_dragon_to_form = true;
                    }
                    return true;
                }
                self._send_ws_message(&CTSMsg::Pass);
                self.state.user_id_to_give_dragon_to = None;
                self.state.show_user_id_to_give_dragon_to_form = false;
                true
            }
            CTSMsgInternal::__Admin_SkipToPlay => {
                self._send_ws_message(&CTSMsg::__Admin_SkipToPlay);
                false
            }
        }
    }

    /// Helper function to actually send the websocket message
    fn _send_ws_message(&mut self, msg: &CTSMsg) {
        match self.ws {
            None => {
                warn!("Can't send message. Websocket is None in state");
            }
            Some(ref mut ws) => {
                let msg = bincode::serialize(&msg).expect("Could not serialize message");
                ws.send_with_u8_array(&msg)
                    .expect("Error sending websocket data as u8 array over websocket");
            }
        }
    }
}
