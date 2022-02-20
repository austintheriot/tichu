use crate::global::{state::AppContext, ws::CTSMsgInternal};
use crate::ui::common::button::{Button, ButtonVariant};
use crate::ui::common::call_small_tichu_button::CallSmallTichuButton;
use crate::ui::common::layout::Layout;
use crate::ui::common::pre_play_hand::PrePlayHand;
use crate::ui::icons::check::Check;
use crate::ui::icons::x::X;
use common::{CallGrandTichuRequest, TichuCallStatus};
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

    let call_grand_tichu_prompt = match app_state.get_current_user_grand_tichu_call_status() {
        Some(call_status @ (TichuCallStatus::Called | TichuCallStatus::Declined)) => html! {
           <>
                <h1>{"Waiting for others to decide..."}</h1>
                <div class="button-container">
                    <Button
                            variant={ButtonVariant::Circle}
                            disabled=true
                            classes={vec!["call-grand-tichu-button".into()]}
                    >
                        {match call_status {
                            TichuCallStatus::Called => html!{
                                <Check classes={vec!["call-grand-tichu-icon".into(), "check".into()]} />
                            },
                            TichuCallStatus::Declined => html!{
                                <X classes={vec!["call-grand-tichu-icon".into(), "x".into()]} />
                            },
                            _ => html!{}
                        }}
                    </Button>
                </div>
           </>
        },
        _ => html! {
            <>
                <h1>{"Call Grand Tichu?"}</h1>
                <p class="info">
                    {"+200 extra points if you go out first."}
                </p>
                <p class="info">
                    {"-200 points if you someone else goes out before you."}
                </p>
                <div class="button-container">
                    <Button
                        variant={ButtonVariant::Circle}
                        onclick={make_handle_call_grand_tichu(CallGrandTichuRequest::Call)}
                        disabled={!app_state.can_call_or_decline_grand_tichu()}
                        classes={vec!["call-grand-tichu-button".into()]}
                    >
                        <Check classes={vec!["call-grand-tichu-icon".into(), "check".into()]} />
                    </Button>
                    <Button
                        variant={ButtonVariant::Circle}
                        onclick={make_handle_call_grand_tichu(CallGrandTichuRequest::Decline)}
                        disabled={!app_state.can_call_or_decline_grand_tichu()}
                        classes={vec!["call-grand-tichu-button".into()]}
                    >
                        <X classes={vec!["call-grand-tichu-icon".into(), "x".into()]} />
                    </Button>
                </div>
            </>
        },
    };

    html! {
        <Layout classes={vec!["grand-tichu-container".to_string()]}>
            {call_grand_tichu_prompt}

            <div class="call-small-tichu-container">
                <CallSmallTichuButton show_optional=true />
                <p>{"± 100 points for going out first"}</p>
            </div>

            <PrePlayHand />
        </Layout>
    }
}
