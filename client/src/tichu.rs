use crate::types::CTSMsgInternal;
use anyhow::Error;
use bincode;
use common::{CTSMsg, CreateGame, GameStage, GameState, JoinGameWithGameCode, STCMsg};
use log::*;
use serde_derive::{Deserialize, Serialize};
use yew::format::{Binary, Json};
use yew::prelude::*;
use yew::services::storage::{Area, StorageService};
use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};

pub struct App {
    link: ComponentLink<Self>,
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
}

const USER_ID_STORAGE_KEY: &str = "yew.tichu.user_id";

const DISPLAY_NAME: &str = "Display Name";

pub enum AppMsg {
    ConnectToWS,
    Disconnected,
    Noop,
    WSMsgReceived(Result<Vec<u8>, Error>),
    SendWSMsg(CTSMsgInternal),
    SetUserId(String),
    SetGameCodeInput(String),
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
        };
        Self {
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
                    self.state.ws_connection_status = "Connected".into();
                }
                true
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
        }
    }

    fn change(&mut self, _prop: Self::Properties) -> ShouldRender {
        false
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
                <button onclick=self.link.callback(|_| AppMsg::SendWSMsg(CTSMsgInternal::Test))>{ "Send test message to server" }</button>
                <br />
                <button onclick=self.link.callback(|_| AppMsg::SendWSMsg(CTSMsgInternal::Ping))>{ "Send ping to server" }</button>
                <br />
                <button onclick=self.link.callback(|_| {AppMsg::SendWSMsg(CTSMsgInternal::CreateGame)})>{ "Create game" }</button>
                <br />
                <input type="text"
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
                warn!("Deserialized data is None. This probably indicates there was an error deserializing the websocket message");
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
                STCMsg::Pong => {}
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
        match msg_type {
            CTSMsgInternal::Test => {
                let msg = CTSMsg::Test(String::from("Hello, server!"));
                self._send_ws_message(&msg);
            }
            CTSMsgInternal::Ping => {
                let msg = CTSMsg::Ping;
                self._send_ws_message(&msg);
            }
            CTSMsgInternal::Pong => {
                let msg = CTSMsg::Pong;
                self._send_ws_message(&msg);
            }
            CTSMsgInternal::CreateGame => {
                let create_game = CreateGame {
                    user_id: self.state.user_id.clone(),
                    display_name: String::from("Example display name"),
                };
                let msg = CTSMsg::CreateGame(create_game);
                self._send_ws_message(&msg);
            }
            CTSMsgInternal::JoinGameWithGameCode => {
                let join_game_with_game_code = JoinGameWithGameCode {
                    game_code: self.state.game_code_input.clone(),
                    display_name: DISPLAY_NAME.into(),
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
