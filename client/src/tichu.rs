use std::collections::HashMap;
use std::hash::Hash;

use log::*;
use serde_derive::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, ToString};
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::MessageEvent;
use yew::format::Json;
use yew::prelude::*;
use yew::services::storage::{Area, StorageService};

pub struct App {
    link: ComponentLink<Self>,
    state: State,
}

pub struct State {
    input_value: String,
    connection_status: String,
    ws: web_sys::WebSocket,
    callbacks: Vec<Callbacks>,
}

pub enum Msg {
    UpdateInput(String),
    UpdateConnectionStatus(String),
    Nope,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
      let (ws, callbacks) = instantiate_ws();
        let state = State {
            input_value: "".into(),
            connection_status: "Connecting...".into(),
            ws,
            callbacks,
        };
        App { link, state }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateInput(val) => {
                info!("New input value: {}", val);
                self.state.input_value = val;
            }
            Msg::UpdateConnectionStatus(val) => {
                info!("New connection status: {}", val);
                self.state.connection_status = val;
            }
            Msg::Nope => {}
        }
        true
    }

    fn view(&self) -> Html {
        info!("rendered!");
        html! {
          <>
          <h1>{ "Test websocket" }</h1>
          <div id="chat">
              <p>{ &self.state.connection_status }</p>
          </div>
          <input class="edit"
            type="text"
            value=&self.state.input_value
            oninput=self.link.callback(move |e: InputData| Msg::UpdateInput(e.value))
            />

          <button type="button" id="send">{ "Send" }</button>
          </>
        }
    }

    fn destroy(&mut self) {
      // clean up callbacks that were assigned to the WebSocket
      for cb in self.state.callbacks.iter() {
        std::mem::drop(cb);
      }
    }
}

#[derive(EnumIter, ToString, Clone, PartialEq, Serialize, Deserialize)]
pub enum Filter {
    All,
    Active,
    Completed,
}

impl<'a> Into<Href> for &'a Filter {
    fn into(self) -> Href {
        match *self {
            Filter::All => "#/".into(),
            Filter::Active => "#/active".into(),
            Filter::Completed => "#/completed".into(),
        }
    }
}

enum Callbacks {
  OnMessage(Closure<dyn FnMut(MessageEvent)>),
  OnError(Closure<dyn FnMut(ErrorEvent)>),
  OnOpen(Closure<dyn FnMut(JsValue)>)
}

fn instantiate_ws() -> (web_sys::WebSocket, Vec<Callbacks>) {
    // let host: String = web_sys::window()
    // .expect("Couldn't get window")
    // .location()
    // .host()
    // .expect("Couldn't get location.host");

    let mut callbacks = Vec::new();
    let host = "localhost:8001";
    let ws_uri = format!("ws://{}/ws?user_id=1234", &host);
    let ws = web_sys::WebSocket::new(&ws_uri).expect("Could not instantiate websocket");
    ws.set_binary_type(web_sys::BinaryType::Arraybuffer);
    let cloned_ws = ws.clone();


    
    let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
      if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
        info!("message event, received Text: {:?}", txt);
      } else {
        info!("message event, received Unknown: {:?}", e.data());
      }
    }) as Box<dyn FnMut(MessageEvent)>);
    // set message event handler on WebSocket
    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    callbacks.push(Callbacks::OnMessage(onmessage_callback));

    let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
      error!("Websocket error occurred: {:?}", e);
    }) as Box<dyn FnMut(ErrorEvent)>);
    ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
    callbacks.push(Callbacks::OnError(onerror_callback));

    let onopen_callback = Closure::wrap(Box::new(move |_| {
        info!("socket opened");
        match cloned_ws.send_with_str("ping") {
            Ok(_) => info!("message successfully sent"),
            Err(err) => info!("error sending message: {:?}", err),
        }
        // send off binary message
        match cloned_ws.send_with_u8_array(&vec![0, 1, 2, 3]) {
            Ok(_) => info!("binary message successfully sent"),
            Err(err) => info!("error sending message: {:?}", err),
        }
    }) as Box<dyn FnMut(JsValue)>);
    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    callbacks.push(Callbacks::OnOpen(onopen_callback));

    (ws, callbacks)
}
