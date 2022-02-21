use super::trade_to_person::TradeToPerson;
use crate::global::{state::AppContext, ws::CTSMsgInternal};
use crate::ui::common::button::{Button, ButtonVariant};
use crate::ui::common::call_small_tichu_container::CallSmallTichuContainer;
use crate::ui::common::layout::Layout;
use crate::ui::common::pre_play_hand::PrePlayHand;
use crate::ui::icons::check::Check;
use crate::ui::icons::up_arrow::UpArrow;
use common::OtherPlayerOption;
use yew::prelude::*;

#[function_component(Trade)]
pub fn trade() -> Html {
    let app_context = use_context::<AppContext>().expect("AppContext not found");
    let app_state = &*app_context.app_reducer_handle;

    let handle_submit = {
        let send_ws_message = app_context.send_ws_message.clone();
        Callback::from(move |_: MouseEvent| {
            send_ws_message.emit(CTSMsgInternal::SubmitTrade);
        })
    };

    html! {
          <Layout classes={vec!["trade".to_string()]}>
              {if !app_state.has_submitted_trade() {
                  html!{
                        <>
                            <h1>{"Trade"}</h1>
                            <Button
                              onclick={handle_submit}
                              disabled={!app_state.can_submit_trade()}
                              button_type="submit"
                              variant={ButtonVariant::Circle}
                            >
                                <UpArrow />
                             </Button>

                            <div class="trade-to-person-container">
                                <TradeToPerson player={OtherPlayerOption::Opponent1} />
                                <TradeToPerson player={OtherPlayerOption::Teammate} />
                                <TradeToPerson player={OtherPlayerOption::Opponent2} />
                            </div>

                            <CallSmallTichuContainer />
                        </>
              }
          } else {
                  html!{
                     <>
                        <p class="waiting">{"Waiting for others to trade..."}</p>
                        <Button
                            disabled=true
                            classes={vec!["check-button".to_string()]}
                            variant={ButtonVariant::Circle}
                        >
                            <Check />
                        </Button>
                     </>
              }
          }}
              <PrePlayHand />
          </Layout>
    }
}
