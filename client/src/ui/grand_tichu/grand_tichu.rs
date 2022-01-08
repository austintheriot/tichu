use super::grand_tichu_status_current_user::GrandTichuStatusCurrentUser;
use crate::global::{state::AppContext, ws::CTSMsgInternal};
use crate::ui::common::call_small_tichu_button::CallSmallTichuButton;
use crate::ui::common::pre_play_hand::PrePlayHand;
use common::CallGrandTichuRequest;
use yew::prelude::*;

#[function_component(GrandTichu)]
pub fn grand_tichu() -> Html {
    let app_context = use_context::<AppContext>().expect("AppContext not found");
    let app_state = &*app_context.app_reducer_handle;
    let make_handle_call_grand_tichu = |call_grand_tichu_request: CallGrandTichuRequest| {
        let send_ws_message = app_context.send_ws_message.clone();
        Callback::from(move |_: MouseEvent| {
            send_ws_message.emit(CTSMsgInternal::CallGrandTichu(
                call_grand_tichu_request.clone(),
            ));
        })
    };

    html! {
          <>
              <h1>{"Grand Tichu"}</h1>
              <GrandTichuStatusCurrentUser />
              <button
                  onclick={make_handle_call_grand_tichu(CallGrandTichuRequest::Call)}
                  disabled={!app_state.can_call_or_decline_grand_tichu()}
              >{"Call Grand Tichu"}</button>
              <button
                  onclick={make_handle_call_grand_tichu(CallGrandTichuRequest::Decline)}
                  disabled={!app_state.can_call_or_decline_grand_tichu()}
              >{"Decline Grand Tichu"}</button>
              <CallSmallTichuButton />
              <p>{"Hand:"}</p>
              <PrePlayHand />
          </>
    }
}
