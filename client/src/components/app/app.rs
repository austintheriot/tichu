use crate::app::state::AppState;
use crate::components::lobby::lobby::Lobby;
use crate::{app::state::AppContext, components::app::ws::use_setup_app_ws};
use yew::{function_component, html, prelude::*};

#[function_component(App)]
pub fn app() -> Html {
    // initialize app state
    let app_reducer_handle = use_reducer_eq(AppState::default);
    // setup websocket and
    use_setup_app_ws(app_reducer_handle.clone());
    // pass app state and websocket down through context
    let context = AppContext {
        app_reducer_handle: app_reducer_handle.clone(),
    };

    html! {
        <ContextProvider<AppContext> {context}>
            <Lobby />
        </ContextProvider<AppContext>>
    }
}
