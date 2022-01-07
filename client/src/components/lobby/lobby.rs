use yew::prelude::*;

use crate::{
    app::state::{AppContext, AppReducerAction},
    components::app::ws::CTSMsgInternal,
};

#[function_component(Lobby)]
pub fn lobby() -> Html {
    let app_context = use_context::<AppContext>().expect("no ctx found");
    let handle_debug = {
        let reducer_handle = app_context.app_reducer_handle.clone();
        Callback::from(move |_: MouseEvent| reducer_handle.dispatch(AppReducerAction::Debug))
    };

    let handle_ping = {
        let send_ws_message = app_context.send_ws_message.clone();
        Callback::from(move |_: MouseEvent| {
            send_ws_message.emit(CTSMsgInternal::Ping);
        })
    };

    html! {
        <>
            <button onclick={handle_debug}>
                {"Pass Debug Action"}
            </button>
            <button onclick={handle_ping}>
                {"Send ping"}
            </button>
        </>
    }
}
