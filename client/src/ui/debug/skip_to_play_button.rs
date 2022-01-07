use yew::prelude::*;

use crate::global::{state::AppContext, ws::CTSMsgInternal};

#[function_component(SkipToPlay)]
pub fn skip_to_play() -> Html {
    let app_context = use_context::<AppContext>().expect("AppContext not found");

    let onclick = {
        let send_ws_message = app_context.send_ws_message.clone();
        Callback::from(move |_: MouseEvent| {
            send_ws_message.emit(CTSMsgInternal::__AdminSkipToPlay);
        })
    };

    html! {
      <button {onclick}>
        {"Skip to Play Stage"}
      </button>
    }
}
