use super::trade_to_person::TradeToPerson;
use crate::global::{state::AppContext, ws::CTSMsgInternal};
use crate::ui::common::call_small_tichu_button::CallSmallTichuButton;
use crate::ui::common::pre_play_hand::PrePlayHand;
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
          <>
              <h1>{"Trade"}</h1>
              <p>{&format!("Has submitted trade: {:?}", app_state.has_submitted_trade())}</p>
              {if !app_state.has_submitted_trade() {
                  html!{
                      <>
                          <button
                              onclick={handle_submit}
                              disabled={!app_state.can_submit_trade()}
                              type="submit">
                          {"Submit"}
                          </button>
                          <br />
                          <br />
                          <TradeToPerson player={OtherPlayerOption::Opponent1} />
                          <br />
                          <TradeToPerson player={OtherPlayerOption::Teammate} />
                          <br />
                          <TradeToPerson player={OtherPlayerOption::Opponent2} />
                          <br />
                          <br />
                          <br />
                          <CallSmallTichuButton />
                          <br />
                          <br />
                          <br />
                      </>
              }
          } else {
                  html!{
                      <p>{"Waiting for others to trade..."}</p>
              }
          }}
              <PrePlayHand />
          </>
    }
}
