use crate::global::{state::AppContext, ws::CTSMsgInternal};
use crate::ui::common::button::Button;
use common::TichuCallStatus;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CallSmallTichuButtonProps {
    #[prop_or_default]
    pub show_optional: bool,
}

#[function_component(CallSmallTichuButton)]
pub fn call_small_tichu_button(props: &CallSmallTichuButtonProps) -> Html {
    let app_context = use_context::<AppContext>().expect("AppContext not found");
    let app_state = &*app_context.app_reducer_handle;
    let handle_send_call_small_tichu = {
        let send_ws_message = app_context.send_ws_message.clone();
        Callback::from(move |_: MouseEvent| {
            send_ws_message.emit(CTSMsgInternal::CallSmallTichu);
        })
    };

    let small_tichu_status = app_state.get_current_user_small_tichu_call_status();

    let button_text_content = match &small_tichu_status {
        Some(TichuCallStatus::Called) => "Called Small Tichu",
        Some(TichuCallStatus::Declined) => "Declined Small Tichu",
        Some(TichuCallStatus::Achieved) => "Achieved Small Tichu",
        Some(TichuCallStatus::Failed) => "Did Not Achieve Small Tichu",
        _ => {
            if props.show_optional {
                "Call Small Tichu (Optional)"
            } else {
                "Call Small Tichu"
            }
        }
    };

    html! {
          <Button
            onclick={handle_send_call_small_tichu}
            disabled={!app_state.can_call_small_tichu()}
            classes={vec!["call-small-tichu-button".to_string()]}
          >
            {button_text_content}
          </Button>
    }
}
