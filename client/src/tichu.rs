use anyhow::Error;
use bincode;
use common::{CTSMsg, STCMsg};
use log::*;
use uuid::Uuid;
use yew::format::{Binary, Json};
use yew::prelude::*;
use yew::services::storage::{Area, StorageService};
use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};

pub struct App {
    ws: Option<WebSocketTask>,
    storage: StorageService,
    user_id: String,
    link: ComponentLink<Self>,
}

const USER_ID_STORAGE_KEY: &str = "yew.tichu.user_id";

pub enum AppMsg {
    ConnectToWS,
    Disconnected,
    Noop,
    WSMsgReceived(Result<Vec<u8>, Error>),
    SendWSMsg(CTSMsg),
    SetUserId(String),
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
        Self {
            ws: None,
            storage,
            user_id,
            link: link,
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
                false
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
                    let url = format!("ws://localhost:8001/ws?user_id={}", self.user_id);
                    let ws_task = WebSocketService::connect_binary(
                        &url,
                        handle_ws_receive_data,
                        handle_ws_update_status,
                    );
                    self.ws = Some(ws_task.unwrap());
                }
                true
            }
            AppMsg::SendWSMsg(msg_type) => handle_ws_message_send(self, msg_type),
            AppMsg::WSMsgReceived(data) => handle_ws_message_received(self, data),
            AppMsg::SetUserId(s) => {
                self.storage.store(USER_ID_STORAGE_KEY, Json(&s));
                self.user_id = s;
                false
            }
        }
    }

    fn change(&mut self, _prop: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <p>{ "Websocket status: "}{ if self.ws.is_none() {"Not connected"} else { "Connected" }} </p>
                <button onclick=self.link.callback(|_| AppMsg::SendWSMsg(CTSMsg::Test(String::from("Hello server!"))))>{ "Send test message to server" }</button>
                <button onclick=self.link.callback(|_| AppMsg::SendWSMsg(CTSMsg::Ping))>{ "Send ping to server" }</button>
            </div>
        }
    }
}

/// Handles when a websocket message is received from the server
/// Returns whether the component should re-render or not
fn handle_ws_message_received(app: &mut App, data: Result<Vec<u8>, Error>) -> bool {
    let should_rerender = true;
    if data.is_err() {
        info!("Data received from websocket was an error {:?}", &data);
        return false;
    }
    let data: STCMsg =
        bincode::deserialize(&data.unwrap()).expect("Could not deserialize message from websocket");
    info!("Received websocket message: {:?}", &data);
    match data {
        STCMsg::Ping => {
            app.link.send_message(AppMsg::SendWSMsg(CTSMsg::Pong));
        }
        STCMsg::Pong => {}
        STCMsg::Test(string) => {}
        STCMsg::UserIdAssigned(s) => {
            info!("New user_id received from websocket!");
            app.link.send_message(AppMsg::SetUserId(s));
        }
        _ => info!("Some other message received!"),
    }
    should_rerender
}

/// Sends a message to the server via websocket
/// Returns whether the component should rerender
fn handle_ws_message_send(app: &mut App, msg_type: CTSMsg) -> bool {
    let should_rerender = false;
    match app.ws {
        None => {
            info!("Can't send message. Websocket is not connected.");
        }
        Some(ref mut ws_task) => {
            info!("Sending websocket message: {:?}", &msg_type);
            match msg_type {
                CTSMsg::Test(s) => {
                    send_ws_message(ws_task, &CTSMsg::Test(s));
                }
                CTSMsg::Ping => {
                    send_ws_message(ws_task, &CTSMsg::Ping);
                }
                CTSMsg::Pong => {
                    send_ws_message(ws_task, &CTSMsg::Pong);
                }
                _ => {
                    info!("Tried to send unexpected message type {:?}", &msg_type);
                }
            }
        }
    }
    should_rerender
}

pub fn send_ws_message(ws_task: &mut WebSocketTask, msg: &CTSMsg) {
    let msg = bincode::serialize(&msg).expect("Could not serialize message");
    ws_task.send_binary(Binary::Ok(msg));
}
