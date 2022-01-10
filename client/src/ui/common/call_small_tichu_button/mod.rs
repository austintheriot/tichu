use crate::global::{state::AppContext, ws::CTSMsgInternal};
use yew::prelude::*;

#[function_component(CallSmallTichuButton)]
pub fn call_small_tichu_button() -> Html {
    let app_context = use_context::<AppContext>().expect("AppContext not found");
    let app_state = &*app_context.app_reducer_handle;
    let handle_send_call_small_tichu = {
        let send_ws_message = app_context.send_ws_message.clone();
        Callback::from(move |_: MouseEvent| {
            send_ws_message.emit(CTSMsgInternal::CallSmallTichu);
        })
    };

    html! {
          <button
            onclick={handle_send_call_small_tichu}
            disabled={!app_state.can_call_small_tichu()}
          >
            {"Call Small Tichu"}
          </button>
    }
}
