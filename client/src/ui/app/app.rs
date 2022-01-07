use crate::global::state::AppContext;
use crate::global::state::AppState;
use crate::global::ws::use_setup_app_ws;
use crate::ui::{debug::debug::Debug, lobby::lobby::Lobby};
use yew::{function_component, html, prelude::*};

#[function_component(App)]
pub fn app() -> Html {
    // initialize app state
    let app_reducer_handle = use_reducer_eq(AppState::default);

    // setup websocket and begin pinging server
    let send_ws_message = use_setup_app_ws(app_reducer_handle.clone());

    // pass app state and a send_ws_message utility down through context
    let context = AppContext {
        app_reducer_handle: app_reducer_handle.clone(),
        send_ws_message,
    };

    html! {
        <ContextProvider<AppContext> {context}>
            <Lobby />
            <Debug />
        </ContextProvider<AppContext>>
    }
}
