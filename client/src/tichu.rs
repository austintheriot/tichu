use anyhow::Error;
use bincode;
use common::CTSMsg;
use log::*;
use yew::binary_format;
use yew::format::Bincode;
use yew::format::{Binary, Json};
use yew::prelude::*;
use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};
use yew::services::ConsoleService;

pub struct App {
    ws: Option<WebSocketTask>,
    link: ComponentLink<Self>,
    text: String,
    server_data: String,
}

pub enum Msg {
    Connect,
    Disconnected,
    Ignore,
    TextInput(String),
    Received(Result<Vec<u8>, Error>),
    SendMsg(CTSMsg),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            ws: None,
            link: link,
            text: String::new(),
            server_data: String::new(),
        }
    }

    fn rendered(&mut self, first_render: bool) {
        // connect to websocket on first render
        if self.ws.is_none() && first_render {
            info!("Sending automatic message to connect");
            self.link.send_message(Msg::Connect);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Connect => {
                info!("Connecting to websocket...");
                let handle_ws_receive_data = self
                    .link
                    .callback(|data: Result<Vec<u8>, Error>| Msg::Received(data));
                let handle_ws_update_status = self.link.callback(|ws_status| {
                    info!("Websocket status: {:?}", ws_status);
                    match ws_status {
                        WebSocketStatus::Closed | WebSocketStatus::Error => Msg::Disconnected,
                        WebSocketStatus::Opened => Msg::Ignore,
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
            Msg::Disconnected => {
                self.ws = None;
                true
            }
            Msg::Ignore => false,
            Msg::TextInput(e) => {
                self.text = e;
                true
            }
            Msg::SendMsg(msg_type) => {
                match self.ws {
                    None => {
                        info!("Can't send message. Websocket is not connected.");
                    }
                    Some(ref mut ws_task) => match msg_type {
                        CTSMsg::Test(s) => {
                            info!("Sending websocket message: {:?}", &s);
                            let s = bincode::serialize(&CTSMsg::Test(s))
                                .expect("Could not serialize message");
                            ws_task.send_binary(Binary::Ok(s));
                        }
                        _ => {
                            info!("Unexpected message type received {:?}", &msg_type);
                        }
                    },
                }
                false
            }
            Msg::Received(data) => {
                if data.is_err() {
                    info!("Data received from server was an error {:?}", &data);
                    return false;
                }
                let data: CTSMsg = bincode::deserialize(&data.unwrap())
                    .expect("Could not deserialize message from server");
                match data {
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
                <button onclick=self.link.callback(|_| Msg::SendMsg(CTSMsg::Test(String::from("Hello server!"))))>{ "Send message to server" }</button>
                // <input type="text" value=self.text.clone() oninput=self.link.callback(|e: InputData| Msg::TextInput(e.value))/>
                <p>{ "Message received from server:" }</p>
                <textarea value=self.server_data.clone()></textarea>
            </div>
        }
    }
}
