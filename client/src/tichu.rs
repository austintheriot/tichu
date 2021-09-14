use log::*;
use anyhow::Error;
use yew::prelude::*;
use yew::format::Json;
use yew::services::ConsoleService;
use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};

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
    SendText,
    Received(Result<String, Error>),
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

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Connect => {
                info!("Connecting to websocket...");
                let handle_ws_receive_data = self.link.callback(|Json(data)| Msg::Received(data));
                let handle_ws_update_status = self.link.callback(|input| {
                    info!("Notification: {:?}", input);
                    match input {
                        WebSocketStatus::Closed | WebSocketStatus::Error => {
                            Msg::Disconnected
                        }
                        _ => Msg::Ignore,
                    }
                });
                if self.ws.is_none() {
                    let ws_task = WebSocketService::connect_text("ws://localhost:8001/ws?user_id=1234", handle_ws_receive_data, handle_ws_update_status);
                    self.ws = Some(ws_task.unwrap());
                }
                true
            }
            Msg::Disconnected => {
                self.ws = None;
                true
            }
            Msg::Ignore => {
                false
            }
            Msg::TextInput(e) => {
                self.text = e;
                true
            }
            Msg::SendText => {
                match self.ws {
                    Some(ref mut ws_task) => {
                        info!("Sending websocket message: {}", &self.text);
                        ws_task.send(Json(&self.text));
                        true
                    }
                    None => {
                        info!("Can't send message. Server is not connected.");
                        false
                    }
                }
            }
            Msg::Received(Ok(s)) => {
                self.server_data.push_str(&format!("{}\n", &s));
                true
            }
            Msg::Received(Err(s)) => {
                self.server_data.push_str(&format!("Error when reading from server: {}\n", &s.to_string()));
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
                <button onclick=self.link.callback(|_| Msg::Connect)>{ "Connect to websocket" }</button>
                <p>{ "Connected: "}{ !self.ws.is_none() } </p>
                <input type="text" value=self.text.clone() oninput=self.link.callback(|e: InputData| Msg::TextInput(e.value))/>
                <button onclick=self.link.callback(|_| Msg::SendText)>{ "Send message to server" }</button>
                <p>{ "Message received from server:" }</p>
                <textarea value=self.server_data.clone()></textarea>
            </div>
        }
    }
}