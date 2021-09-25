use std::mem::discriminant;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::types::CTSMsgInternal;
use anyhow::Error;
use bincode;
use common::{
    clean_up_display_name, clean_up_game_code, validate_display_name, validate_game_code,
    validate_team_name, CTSMsg, CreateGame, GameStage, JoinGameWithGameCode, PublicGameState,
    PublicUser, STCMsg, Team, NO_USER_ID,
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

    game_code_input: String,
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
    SetGameCodeInput(String),
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
            game_code_input: "".into(),
            is_alive: false,
            team_a_name_input: "".into(),
            team_b_name_input: "".into(),
        };
        Self {
            interval_task: None,
            ws: None,
            storage,
            link: link,
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
                let handle_ws_receive_data = self
                    .link
                    .callback(|data: Result<Vec<u8>, Error>| AppMsg::WSMsgReceived(data));
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
                        match *app_reference_guard {
                            // It is unsafe to dereference raw pointer, but the pointer
                            // should be valid as long as the App component is mounted.
                            // The biggest risk here is trying to send a message while the
                            // app is being mutated by some other message.
                            Some(app) => unsafe {
                                (*app)
                                    .link
                                    .send_message(AppMsg::SendWSMsg(CTSMsgInternal::Ping));
                            },
                            None => {}
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
            AppMsg::SetGameCodeInput(s) => {
                let s = clean_up_game_code(&s);
                self.state.game_code_input = s;
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
        let app_reference = app_reference_guard.take();
        drop(app_reference);
    }

    fn view(&self) -> Html {
        html! {
            <div>
                // Info -----------------------------------------------------------------------------------
                <h1> { "Info" } </h1>
                <p>{ "Websocket Status: "}{ match &self.state.ws_connection_status {
                    WSConnectionStatus::Connected => "Connected",
                    WSConnectionStatus::NotConnected => "Not Connected"
                } } </p>
                <p> {"Stage: " }
                { if let Some(game_state) = &self.state.game_state {
                        match game_state.stage {
                            GameStage::Lobby => {
                                "Lobby"
                            },
                            _ => "Other",
                        }
                    } else {
                        "No game state"
                }}
                </p>
                <p> { "Game Code: " } {if let Some(game_state) = &self.state.game_state {
                    &game_state.game_code
                } else {
                    ""
                } } </p>
                <p> { "Participants: " } { self.view_participants() } </p>
                <p> { "Owner: " } { self.view_owner() } </p>
                <p> { "Teams: " } { self.view_teams() } </p>
                <button onclick=self.link.callback(|_| AppMsg::SendWSMsg(CTSMsgInternal::Test))>{ "Send test message to server" }</button>
                <br />
                <button onclick=self.link.callback(|_| AppMsg::SendWSMsg(CTSMsgInternal::Ping))>{ "Send ping to server" }</button>
                <br />
                <br />
                 // Lobby -----------------------------------------------------------------------------------
                 <h1> { "Lobby" } </h1>
                <label for="display-name-input"> { "Display Name" } </label>
                <br />
                <input
                    id="display-name-input"
                    type="text"
                    value=self.state.display_name_input.clone()
                    oninput=self.link.callback(|e: InputData| AppMsg::SetDisplayNameInput(e.value))/>
                <br />
                <label for="game-code-input"> { "Game Code" } </label>
                <br />
                <br />
                <input
                    id="game-code-input"
                    type="text"
                    value=self.state.game_code_input.clone()
                    oninput=self.link.callback(|e: InputData| AppMsg::SetGameCodeInput(e.value))/>
                <br />
                <br />
                <button
                    onclick=self.link.callback(|_| {AppMsg::SendWSMsg(CTSMsgInternal::CreateGame)})
                    disabled=!self.can_create_game()
                    >{ "Create game" }</button>
                <br />
                <button
                    onclick=self.link.callback(|_| {AppMsg::SendWSMsg(CTSMsgInternal::JoinGameWithGameCode)})
                    disabled=!self.can_join_game()
                    >{ "Join game" }</button>
                <br />
                <button
                    onclick=self.link.callback(|_| {AppMsg::SendWSMsg(CTSMsgInternal::LeaveGame)})
                    disabled=!self.can_leave_game()
                    >{ "Leave game" }</button>
                <br />
                <br />
                <hr />
                <br />
                <br />
                // Teams -----------------------------------------------------------------------------------
                <h1> { "Teams" } </h1>
                <label for="team-a-name-input"> { "Team A Name" } </label>
                <br />
                <input
                    id="team-a-name-input"
                    disabled=!self.is_team_stage() || self.is_on_team_b()
                    type="text"
                    value=self.state.team_a_name_input.clone()
                    onblur=self.link.callback(|_| AppMsg::SendWSMsg(CTSMsgInternal::RenameTeamA))
                    oninput=self.link.callback(|e: InputData| AppMsg::SetTeamANameInput(e.value))/>
                <br />
                <button
                    onclick=self.link.callback(|_| {AppMsg::SendWSMsg(CTSMsgInternal::MoveToTeamA)})
                    disabled=!self.is_team_stage() || self.is_on_team_a()
                    >{ "Move to Team A" }</button>
                <br />
                <button
                    onclick=self.link.callback(|_| {AppMsg::SendWSMsg(CTSMsgInternal::MoveToTeamB)})
                    disabled=!self.is_team_stage() || self.is_on_team_b()
                    >{ "Move to Team B" }</button>
                <br />
                <input
                    id="team-b-name-input"
                    disabled=!self.is_team_stage() || self.is_on_team_a()
                    type="text"
                    value=self.state.team_b_name_input.clone()
                    onblur=self.link.callback(|_| AppMsg::SendWSMsg(CTSMsgInternal::RenameTeamB))
                    oninput=self.link.callback(|e: InputData| AppMsg::SetTeamBNameInput(e.value))/>
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
            && validate_game_code(&self.state.game_code_input).is_none()
    }

    fn can_leave_game(&self) -> bool {
        self.ws.is_some()
            && self.state.game_state.is_some()
            && match self.state.game_state.as_ref().unwrap().stage {
                GameStage::Lobby => true,
                _ => false,
            }
    }

    fn is_team_stage(&self) -> bool {
        match &self.state.game_state {
            None => false,
            Some(game_state) => match &game_state.stage {
                GameStage::Teams(_) => true,
                _ => false,
            },
        }
    }

    fn is_on_team_a(&self) -> bool {
        match &self.state.game_state {
            None => false,
            Some(game_state) => match &game_state.stage {
                GameStage::Teams(teams) => teams
                    .0
                    .user_ids
                    .iter()
                    .find(|participant_id| **participant_id == self.state.user_id)
                    .is_some(),
                _ => false,
            },
        }
    }

    fn is_on_team_b(&self) -> bool {
        match &self.state.game_state {
            None => false,
            Some(game_state) => match &game_state.stage {
                GameStage::Teams(teams) => teams
                    .1
                    .user_ids
                    .iter()
                    .find(|participant_id| **participant_id == self.state.user_id)
                    .is_some(),
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

    fn view_team(&self, team: &Team) -> Html {
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

    fn view_teams(&self) -> Html {
        if let Some(game_state) = &self.state.game_state {
            match &game_state.stage {
                GameStage::Teams(team_state) => {
                    html! {
                        <ul>
                            <li> {self.view_team(&team_state.0)} </li>
                            <li> {self.view_team(&team_state.1)} </li>
                        </ul>
                    }
                }
                _ => html! { <></> },
            }
        } else {
            html! { <></> }
        }
    }

    fn view_owner(&self) -> Html {
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
                STCMsg::GameState(game_state) => {
                    self.state.game_state = game_state;
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
                STCMsg::Test(_) => {}
                STCMsg::GameCreated(_) => {}
                STCMsg::UserJoined(_) => {}
                STCMsg::UserDisconnected(_) => {}
                STCMsg::UserReconnected(_) => {}
                STCMsg::UserLeft(_) => {}
                STCMsg::OwnerReassigned(_) => {}
                STCMsg::UserMovedToTeamA(_) => {}
                STCMsg::UserMovedToTeamB(_) => {}
                _ => warn!("Unexpected websocket message received {:#?}", data),
            },
        }

        should_rerender
    }

    /// Sends a message to the server via websocket
    /// Returns whether the component should rerender
    fn send_ws_message(&mut self, msg_type: CTSMsgInternal) -> bool {
        let should_rerender = false;
        info!("Sending websocket message: {:#?}", msg_type);
        match msg_type {
            CTSMsgInternal::Test => {
                self._send_ws_message(&CTSMsg::Test(String::from("Hello, server!")))
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

                if should_reconnect {
                    self.state.is_alive = false;
                    self.link.send_message(AppMsg::ConnectToWS);
                } else {
                    self.state.is_alive = false;
                    self._send_ws_message(&CTSMsg::Ping);
                }
            }
            CTSMsgInternal::Pong => self._send_ws_message(&CTSMsg::Pong),
            CTSMsgInternal::CreateGame => {
                if !self.can_create_game() {
                    return false;
                }

                // save display name input to state/localStorage
                self.link.send_message(AppMsg::SetDisplayName(
                    self.state.display_name_input.clone(),
                ));

                let create_game = CreateGame {
                    user_id: self.state.user_id.clone(),
                    display_name: self.state.display_name_input.clone(),
                };
                let msg = CTSMsg::CreateGame(create_game);
                self._send_ws_message(&msg);
            }
            CTSMsgInternal::JoinGameWithGameCode => {
                if !self.can_join_game() {
                    return false;
                }

                // save display name input to state/localStorage
                self.link.send_message(AppMsg::SetDisplayName(
                    self.state.display_name_input.clone(),
                ));

                let join_game_with_game_code = JoinGameWithGameCode {
                    game_code: self.state.game_code_input.clone().to_uppercase(),
                    display_name: self.state.display_name_input.clone(),
                    user_id: self.state.user_id.clone(),
                };
                let msg = CTSMsg::JoinGameWithGameCode(join_game_with_game_code);
                self._send_ws_message(&msg);
            }
            CTSMsgInternal::LeaveGame => {
                if !self.can_leave_game() {
                    return false;
                }

                self._send_ws_message(&CTSMsg::LeaveGame);
            }
            CTSMsgInternal::MoveToTeamA => {
                self._send_ws_message(&CTSMsg::MoveToTeamA);
            }
            CTSMsgInternal::MoveToTeamB => {
                self._send_ws_message(&CTSMsg::MoveToTeamB);
            }
            CTSMsgInternal::RenameTeamA => {
                if validate_team_name(&self.state.team_a_name_input).is_some() {
                    return false;
                }
                self._send_ws_message(&CTSMsg::RenameTeamA(self.state.team_a_name_input.clone()));
            }
            CTSMsgInternal::RenameTeamB => {
                if validate_team_name(&self.state.team_b_name_input).is_some() {
                    return false;
                }
                self._send_ws_message(&CTSMsg::RenameTeamB(self.state.team_b_name_input.clone()));
            }
            _ => {
                warn!("Tried to send unexpected message type {:?}", &msg_type);
            }
        }
        should_rerender
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
