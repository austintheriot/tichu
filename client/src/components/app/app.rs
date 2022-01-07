use crate::app::state::use_setup_app_reducer;
use crate::components::lobby::Lobby;
use crate::{app::state::AppContext, components::app::ws::use_setup_app_ws};
use yew::{function_component, html, prelude::*};

#[function_component(App)]
pub fn app() -> Html {
    let app_reducer_handle = use_setup_app_reducer();
    use_setup_app_ws(app_reducer_handle.clone());
    let context = AppContext {
        reducer_handle: app_reducer_handle.clone(),
    };

    html! {
        <ContextProvider<AppContext> {context}>
            <Lobby />
        </ContextProvider<AppContext>>
    }
}
