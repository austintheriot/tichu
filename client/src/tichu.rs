use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::types::CTSMsgInternal;
use anyhow::Error;
use bincode;
use common::{CTSMsg, CreateGame, GameStage, GameState, JoinGameWithGameCode, STCMsg};
use log::*;
use serde_derive::{Deserialize, Serialize};
use yew::format::{Binary, Json};
use yew::prelude::*;
use yew::services::interval::IntervalTask;
use yew::services::storage::{Area, StorageService};
use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};
use yew::services::IntervalService;

pub struct App {
    link: ComponentLink<Self>,
    interval_task: Option<IntervalTask>,
    ws: Option<WebSocketTask>,
    storage: StorageService,
    state: State,
}

#[derive(Serialize, Deserialize)]
struct State {
    ws_connection_status: String,
    user_id: String,
    game_state: Option<GameState>,
    game_code_input: String,
    display_name_input: String,
    is_alive: bool,
}

const USER_ID_STORAGE_KEY: &str = "yew.tichu.user_id";

pub enum AppMsg {
    ConnectToWS,
    BeginPing,
    Disconnected,
    Noop,
    WSMsgReceived(Result<Vec<u8>, Error>),
    SendWSMsg(CTSMsgInternal),
    SetUserId(String),
    SetGameCodeInput(String),
    SetDisplayNameInput(String),
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
        let user_id = {
            if let Json(Ok(restored_user_id)) = storage.restore(USER_ID_STORAGE_KEY) {
                restored_user_id
            } else {
                storage.store(USER_ID_STORAGE_KEY, Json(&common::NO_USER_ID));
                String::from(common::NO_USER_ID)
            }
        };
        let state = State {
            ws_connection_status: "Not connected".into(),
            user_id,
            game_state: None,
            game_code_input: "".into(),
            display_name_input: "".into(),
            is_alive: false,
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
                self.state.ws_connection_status = "Disconnected".into();
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
                    );
                    self.ws = Some(ws_task.unwrap());
                    self.state.is_alive = true;
                    self.state.ws_connection_status = "Connected".into();
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
            AppMsg::SetGameCodeInput(s) => {
                self.state.game_code_input = s;
                true
            }
            AppMsg::SetDisplayNameInput(s) => {
                self.state.display_name_input = s;
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
                <p>{ "Websocket Status: "}{ &self.state.ws_connection_status } </p>
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
                <label for="display-name-input"> { "Display Name" } </label>
                <input
                    id="display-name-input"
                    type="text"
                    value=self.state.display_name_input.clone()
                    oninput=self.link.callback(|e: InputData| AppMsg::SetDisplayNameInput(e.value))/>
                <button onclick=self.link.callback(|_| AppMsg::SendWSMsg(CTSMsgInternal::Test))>{ "Send test message to server" }</button>
                <br />
                <button onclick=self.link.callback(|_| AppMsg::SendWSMsg(CTSMsgInternal::Ping))>{ "Send ping to server" }</button>
                <br />
                <button onclick=self.link.callback(|_| {AppMsg::SendWSMsg(CTSMsgInternal::CreateGame)})>{ "Create game" }</button>
                <br />
                <label for="game-code-input"> { "Game Code" } </label>
               <input
                    id="game-code-input"
                    type="text"
                    value=self.state.game_code_input.clone()
                    oninput=self.link.callback(|e: InputData| AppMsg::SetGameCodeInput(e.value))/>
                <button onclick=self.link.callback(|_| {AppMsg::SendWSMsg(CTSMsgInternal::JoinGameWithGameCode)})>{ "Join game" }</button>
                <br />
            </div>
        }
    }
}

impl App {
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
                    self.state.game_state = Some(game_state);
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
                _ => warn!("Unexpected websocket message received."),
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
                    self.state.is_alive = false;
                    let ws = self.ws.take();
                    drop(ws);
                }

                if should_reconnect {
                    // need to try reconnecting
                    self.link.send_message(AppMsg::ConnectToWS);
                } else {
                    self.state.is_alive = false;
                    self._send_ws_message(&CTSMsg::Ping);
                }
            }
            CTSMsgInternal::Pong => self._send_ws_message(&CTSMsg::Pong),
            CTSMsgInternal::CreateGame => {
                let create_game = CreateGame {
                    user_id: self.state.user_id.clone(),
                    display_name: self.state.display_name_input.clone(),
                };
                let msg = CTSMsg::CreateGame(create_game);
                self._send_ws_message(&msg);
            }
            CTSMsgInternal::JoinGameWithGameCode => {
                let join_game_with_game_code = JoinGameWithGameCode {
                    game_code: self.state.game_code_input.clone(),
                    display_name: self.state.display_name_input.clone(),
                    user_id: self.state.user_id.clone(),
                };
                let msg = CTSMsg::JoinGameWithGameCode(join_game_with_game_code);
                self._send_ws_message(&msg);
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
