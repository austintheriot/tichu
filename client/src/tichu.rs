use anyhow::Error;
use bincode;
use common::CTSMsg;
use log::*;
use yew::format::{Binary, Json};
use yew::prelude::*;
use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};

pub struct App {
    ws: Option<WebSocketTask>,
    link: ComponentLink<Self>,
    server_data: String,
}

pub enum AppMsg {
    ConnectToWS,
    Disconnected,
    Noop,
    WSMsgReceived(Result<Vec<u8>, Error>),
    SendWSMsg(CTSMsg),
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            ws: None,
            link: link,
            server_data: String::new(),
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
                    let ws_task = WebSocketService::connect_binary(
                        "ws://localhost:8001/ws?user_id=1234",
                        handle_ws_receive_data,
                        handle_ws_update_status,
                    );
                    self.ws = Some(ws_task.unwrap());
                }
                true
            }
            AppMsg::SendWSMsg(msg_type) => {
                match self.ws {
                    None => {
                        info!("Can't send message. Websocket is not connected.");
                    }
                    Some(ref mut ws_task) => {
                        info!("Sending websocket message: {:?}", &msg_type);
                        match msg_type {
                        CTSMsg::Test(s) => {
                            let s = bincode::serialize(&CTSMsg::Test(s))
                                .expect("Could not serialize message");
                            ws_task.send_binary(Binary::Ok(s));
                        }
                        CTSMsg::Ping => {
                            let s = bincode::serialize(&CTSMsg::Ping)
                                .expect("Could not serialize message");
                            ws_task.send_binary(Binary::Ok(s));
                        }
                        _ => {
                            info!("Unexpected message type received {:?}", &msg_type);
                        }
                    }},
                }
                false
            }
            AppMsg::WSMsgReceived(data) => {
                if data.is_err() {
                    info!("Data received from websocket was an error {:?}", &data);
                    return false;
                }
                let data: CTSMsg = bincode::deserialize(&data.unwrap())
                    .expect("Could not deserialize message from websocket");
                info!("Received websocket message: {:?}", &data);
                match data {
                    CTSMsg::Ping => {
                        self.link.send_message(AppMsg::SendWSMsg(CTSMsg::Pong));
                    }
                    CTSMsg::Pong => {
                        info!("Pong received from websocket!")
                    }
                    CTSMsg::Test(string) => {
                        info!("Test message received! Message: {}", string);
                    }
                    _ => info!("Some other message received!"),
                }
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
                <p>{ "Websocket status: "}{ if self.ws.is_none() {"Connecting..."} else { "Connected" }} </p>
                <button onclick=self.link.callback(|_| AppMsg::SendWSMsg(CTSMsg::Test(String::from("Hello server!"))))>{ "Send test message to server" }</button>
                <button onclick=self.link.callback(|_| AppMsg::SendWSMsg(CTSMsg::Ping))>{ "Send ping to server" }</button>
                // <input type="text" value=self.text.clone() oninput=self.link.callback(|e: InputData| AppMsg::TextInput(e.value))/>
                <p>{ "Message received from server:" }</p>
                <textarea value=self.server_data.clone()></textarea>
            </div>
        }
    }
}
