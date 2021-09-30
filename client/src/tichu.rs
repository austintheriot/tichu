use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::types::CTSMsgInternal;
use anyhow::Error;
use common::{
    clean_up_display_name, clean_up_game_code, validate_display_name, validate_game_code,
    validate_team_name, CTSMsg, CallGrandTichuRequest, CreateGame, JoinGameWithGameCode,
    MutableTeam, PublicGameStage, PublicGameState, PublicUser, RenameTeam, STCMsg, TeamOption,
    TichuCallStatus, NO_USER_ID,
};
use log::*;
use serde_derive::{Deserialize, Serialize};
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
            storage.store(USER_ID_STORAGE_KEY, Json(&NO_USER_ID));
            String::from(NO_USER_ID)
        };

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
                    let url = format!("ws://localhost:8001/ws?user_id={}", self.state.user_id);
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
                { self.view_debug() }
                <br />
                <br />
                <hr />
                <br />
                { match &self.state.game_state {
                    None => self.view_join(),
                    Some(game_state) => {
                        match game_state.stage {
                            PublicGameStage::Lobby => self.view_lobby(),
                            PublicGameStage::Teams(_) => self.view_teams(),
                            PublicGameStage::PublicGrandTichu(_) => self.view_grand_tichu(),
                            _ => html!{ <> </> }
                        }
                    }
                }}
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
        matches!(
            self.state.game_state.as_ref().unwrap().stage,
            PublicGameStage::PublicGrandTichu(_)
        )
    }

    fn can_call_small_tichu(&self) -> bool {
        // hasn't yet played a card
        let has_played_first_card = if let Some(game_state) = &self.state.game_state {
            game_state.current_user.has_played_first_card
        } else {
            false
        };

        // is undecided about small tichu
        let (undecided_about_small_tichu, game_stage_is_active) =
            if let Some(game_state) = &self.state.game_state {
                if let PublicGameStage::PublicGrandTichu(public_grand_tichu) = &game_state.stage {
                    if let Some(call_status) = (*public_grand_tichu).small_tichus.iter().find(
                        |user_id_with_tich_call_status| {
                            *user_id_with_tich_call_status.user_id == *self.state.user_id
                        },
                    ) {
                        (
                            matches!(call_status.tichu_call_status, TichuCallStatus::Undecided),
                            true,
                        )
                    } else {
                        (true, true)
                    }
                } else {
                    // user undecided but game stage is not active (TODO: update)
                    (true, false)
                }
            } else {
                // user undecided but game stage is not active (TODO: update)
                (true, false)
            };

        return !has_played_first_card && undecided_about_small_tichu && game_stage_is_active;
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
                { for game_state.participants.iter().map(|user| {
                    html! {
                        <li> { &user.display_name } </li>
                    }
                })}
                </ul>
            }
        } else {
            html! { <></> }
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
                <li> { "Team Name: "} {{&team.team_name}} </li>
                <li> { "Score: "} {{&team.score}} </li>
                <li> { "Users: "} { for team.user_ids.iter().map(|id| {
                    html!{
                        <p> { match &self.get_participant_by_id(id) {
                            Some(participant) => &participant.display_name,
                            None => ""
                        }} </p>
                    }
                })} </li>
            </ul>
        }
    }

    fn debug_teams(&self) -> Html {
        if let Some(game_state) = &self.state.game_state {
            match &game_state.stage {
                PublicGameStage::Teams(team_state) => {
                    html! {
                        <ul>
                            <li> {self.debug_team(&team_state[0])} </li>
                            <li> {self.debug_team(&team_state[1])} </li>
                        </ul>
                    }
                }
                _ => html! { <></> },
            }
        } else {
            html! { <></> }
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
                            <li> { &owner.display_name } </li>
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
            html! { <></> }
        }
    }

    fn view_debug_grand_tichu_for_user(&self, user_id: &str) -> Html {
        let grand_tichu_call_status = match &self.state.game_state {
            Some(game_state) => match &game_state.stage {
                PublicGameStage::PublicGrandTichu(grand_tichu_state) => {
                    match grand_tichu_state.grand_tichus.iter().find(
                        |user_id_with_tichu_call_status| {
                            *user_id_with_tichu_call_status.user_id == *user_id
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
            <p> { &format!("Grand Tichu Call Status for {} ----- ", user_id)} {grand_tichu_call_status} { "\n" }</p>
        }
    }

    fn view_debug_small_tichu_for_user(&self, user_id: &str) -> Html {
        let small_tichu_call_status = match &self.state.game_state {
            Some(game_state) => match &game_state.stage {
                PublicGameStage::PublicGrandTichu(grand_tichu_state) => {
                    match grand_tichu_state.small_tichus.iter().find(
                        |user_id_with_tichu_call_status| {
                            *user_id_with_tichu_call_status.user_id == *user_id
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
            <p> { &format!("Small Tichu Call Status for {} ----- ", user_id)} {small_tichu_call_status} { "\n" }</p>
        }
    }

    fn view_debug_all_participants_grand_tichu(&self) -> Html {
        match &self.state.game_state {
            Some(game_state) => html! {{
            for game_state
                .participants
                .iter()
                .map(|user| self.view_debug_grand_tichu_for_user(&user.user_id))
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
                .map(|user| self.view_debug_small_tichu_for_user(&user.user_id))
            }},
            None => html! {<> </>},
        }
    }

    fn view_debug(&self) -> Html {
        html! {
            <>
            <h1> { "Debug Info:" } </h1>
                <p> { "Display Name: " } { &self.state.display_name }</p>
                <p> { "User ID: " } { &self.state.user_id }</p>
                <p>{ "Websocket Status: "}{ match &self.state.ws_connection_status {
                    WSConnectionStatus::Connected => "Connected",
                    WSConnectionStatus::NotConnected => "Not Connected"
                } } </p>
                <p> { "Game Code: " } {
                    if let Some(game_state) = &self.state.game_state {
                        &game_state.game_code
                    } else {
                        ""
                    }}
                </p>
                <p> { "Participants: " } { self.view_participants() } </p>
                <p> { "Owner: " } { self.debug_owner() } </p>
                <p> { "Teams: " } { self.debug_teams() } </p>
                { self.view_debug_all_participants_grand_tichu() }
                { self.view_debug_all_participants_small_tichu() }
                <button onclick=self.link.callback(|_| AppMsg::SendWSMsg(CTSMsgInternal::Test))>{ "Send test message to server" }</button>
                <br />
                <button onclick=self.link.callback(|_| AppMsg::SendWSMsg(CTSMsgInternal::Ping))>{ "Send ping to server" }</button>
            </>
        }
    }

    fn view_join(&self) -> Html {
        html! {
            <>
            <h1> { "Tichu" } </h1>
                <form onsubmit=self.link.callback(|e: FocusEvent| {
                    e.prevent_default();
                    AppMsg::SendWSMsg(CTSMsgInternal::JoinGameWithGameCode)
                })>
                    <label for="join-room-display-name-input"> { "Display Name" } </label>
                    <br />
                    <input
                        id="join-room-display-name-input"
                        type="text"
                        value=self.state.display_name_input.clone()
                        oninput=self.link.callback(|e: InputData| AppMsg::SetDisplayNameInput(e.value))/>
                    <br />
                    <label for="join-room-game-code-input"> { "Game Code" } </label>
                    <br />
                    <input
                        id="join-room-game-code-input"
                        type="text"
                        value=self.state.join_room_game_code_input.clone()
                        oninput=self.link.callback(|e: InputData| AppMsg::SetJoinRoomGameCodeInput(e.value))/>
                    <br />
                    <button
                        type="submit"
                        onclick=self.link.callback(|_| {AppMsg::SendWSMsg(CTSMsgInternal::JoinGameWithGameCode)})
                        disabled=!self.can_join_game()
                        >{ "Join game" }</button>
                </form>
                <br />
                <br />
                <form onsubmit=self.link.callback(|e: FocusEvent| {
                    e.prevent_default();
                    AppMsg::SendWSMsg(CTSMsgInternal::CreateGame)
                })>
                    <label for="join-room-display-name-input"> { "Display Name" } </label>
                    <br />
                    <input
                        id="create-room-display-name-input"
                        type="text"
                        value=self.state.display_name_input.clone()
                        oninput=self.link.callback(|e: InputData| AppMsg::SetDisplayNameInput(e.value))/>
                    <br />
                    <button
                        type="submit"
                        onclick=self.link.callback(|_| {AppMsg::SendWSMsg(CTSMsgInternal::CreateGame)})
                        disabled=!self.can_create_game()
                        >{ "Create game" }</button>
                </form>
            </>
        }
    }

    fn view_lobby(&self) -> Html {
        html! {
            <>
            <h1> { "Lobby" } </h1>
            <h2> { "Game Code: " } {
                if let Some(game_state) = &self.state.game_state {
                    &game_state.game_code
                } else {
                    ""
                }
            } </h2>
                <button
                onclick=self.link.callback(|_| {AppMsg::SendWSMsg(CTSMsgInternal::LeaveGame)})
                disabled=!self.can_leave_game()
                >{ "Leave game" }</button>
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
            <h1> { "Teams" } </h1>
                <label for="team-a-name-input"> { "Team A Name" } </label>
                <br />
                <form onsubmit=self.link.callback(|e: FocusEvent | {
                    e.prevent_default();
                    AppMsg::SendWSMsg(CTSMsgInternal::RenameTeam(TeamOption::TeamA))
                })>
                    <label for="team-a-name-input">{ "Team Name" }</label>
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
                    >{ "Move to Team A" }</button>
                <br />
                <button
                    onclick=self.link.callback(|_| {AppMsg::SendWSMsg(CTSMsgInternal::MoveToTeam(TeamOption::TeamB))})
                    disabled=!self.is_team_stage() || self.is_on_team_b()
                    >{ "Move to Team B" }</button>
                <br />
                <form onsubmit=self.link.callback(|e: FocusEvent | {
                    e.prevent_default();
                    AppMsg::SendWSMsg(CTSMsgInternal::RenameTeam(TeamOption::TeamB))
                })>
                    <label for="team-b-name-input">{ "Team Name" }</label>
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
                    > { "Start" } </button>
                  }
               } else {
                   html!{
                       <> </>
                   }
               }}
            </>
        }
    }

    fn view_hand(&self) -> Html {
        match &self.state.game_state {
            Some(game_state) => {
                let mut sorted_hand = game_state.current_user.hand.clone();
                sorted_hand.sort();
                html! {
                    <ul>
                        { for sorted_hand.iter().map(|card| {
                            html!{
                                <li> { &format!("{:#?}", card) } </li>
                            }
                        })}
                    </ul>
                }
            }
            None => html! {
                <></>
            },
        }
    }

    fn view_grand_tichu(&self) -> Html {
        html! {
            <>
                <h1> { "Grand Tichu" } </h1>
                <button
                    onclick=self.link.callback(|_| {AppMsg::SendWSMsg(CTSMsgInternal::CallGrandTichu(CallGrandTichuRequest::Call))})
                    disabled=!self.can_call_or_decline_grand_tichu()
                > { "Call Grand Tichu" } </button>
                <button
                    onclick=self.link.callback(|_| {AppMsg::SendWSMsg(CTSMsgInternal::CallGrandTichu(CallGrandTichuRequest::Decline))})
                    disabled=!self.can_call_or_decline_grand_tichu()
                > { "Decline Grand Tichu" } </button>
                <button
                    onclick=self.link.callback(|_| {AppMsg::SendWSMsg(CTSMsgInternal::CallSmallTichu)})
                    disabled=!self.can_call_small_tichu()
                > { "Call Small Tichu" } </button>
                <p> { "Hand:" } </p>
                { self.view_hand() }
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
                    // if team names are empty, update team name inputs to reflect state
                    if self.state.team_a_name_input.is_empty()
                        || self.state.team_b_name_input.is_empty()
                    {
                        if let Some(new_game_state) = &*new_game_state {
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

                    // save display name input to state/localStorage
                    match &*new_game_state {
                        None => {}
                        Some(new_game_state) => {
                            self.link.send_message(AppMsg::SetDisplayName(
                                (*new_game_state.current_user.display_name).to_string(),
                            ));
                        }
                    }

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
                STCMsg::GameCreated(_) => {}
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

                let create_game = CreateGame {
                    user_id: self.state.user_id.clone(),
                    display_name: self.state.display_name_input.clone(),
                };

                let msg = CTSMsg::CreateGame(create_game);
                self._send_ws_message(&msg);
                false
            }
            CTSMsgInternal::JoinGameWithGameCode => {
                if !self.can_join_game() {
                    return false;
                }

                let join_game_with_game_code = JoinGameWithGameCode {
                    game_code: self.state.join_room_game_code_input.clone().to_uppercase(),
                    display_name: self.state.display_name_input.clone(),
                    user_id: self.state.user_id.clone(),
                };

                let msg = CTSMsg::JoinGameWithGameCode(join_game_with_game_code);
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

                self._send_ws_message(&CTSMsg::RenameTeam(RenameTeam {
                    team_name: team_name_input_clone,
                    team_option,
                }));

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
