use super::cards_on_table::CardsOnTable;
use super::choose_opponent_input::ChooseOpponentInput;
use super::play_hand::PlayHand;
use super::selected_play_cards::SelectedPlayCards;
use super::tricks::Tricks;
use super::turn_display_name::TurnDisplayName;
use super::wish_for_card_input::WishForCardInput;
use super::wished_for_card::WishedForCard;
use crate::global::{state::AppContext, ws::CTSMsgInternal};
use crate::ui::common::call_small_tichu_button::CallSmallTichuButton;
use common::{get_card_combination, DRAGON, MAH_JONG};
use yew::prelude::*;

#[function_component(Play)]
pub fn play() -> Html {
    let app_context = use_context::<AppContext>().expect("AppContext not found");
    let app_state = &*app_context.app_reducer_handle;

    let handle_submit_play_cards = {
        let send_ws_message = app_context.send_ws_message.clone();
        Callback::from(move |_: MouseEvent| {
            send_ws_message.emit(CTSMsgInternal::PlayCards);
        })
    };

    let handle_pass = {
        let send_ws_message = app_context.send_ws_message.clone();
        Callback::from(move |_: MouseEvent| {
            send_ws_message.emit(CTSMsgInternal::Pass);
        })
    };

    html! {
          <>
            <h1>{"Play"}</h1>
            <br />
            <br />
            <TurnDisplayName />
            <br />
            <br />
            <CardsOnTable />
            <br />
            <br />
            <WishedForCard />
            {if app_state.selected_play_cards.contains(&MAH_JONG) {
                html!{ <WishForCardInput /> }
            } else {
                html!{}
            }}
            <br />
            <br />
            {if app_state.selected_play_cards.contains(&DRAGON) {
               html!{ <ChooseOpponentInput /> }
            } else {
                html!{}
            }}
            <br />
            <br />
            {if app_state.selected_play_cards.len() == 0 {
               html!{
                <button
                    onclick={handle_pass}
                    type="button"
                    disabled={!app_state.get_can_pass()}
                    >
                    {"Pass"}
                </button>
               }
            } else {
                html!{
                    <button
                        disabled={!app_state.can_play_cards()}
                        onclick={handle_submit_play_cards}
                        type="button"
                        >
                        {"Submit cards"}
                    </button>
                }
            }}
            <br />
            <br />
            <CallSmallTichuButton />
            <br />
            <br />
            <p>
              {if get_card_combination(app_state
                  .get_prev_played_combo(),
                  &app_state.selected_play_cards,
                  &app_state.user_id
                ).is_some() {
                  "Valid combination"
              } else {
                  "Invalid combination"
              }}
            </p>
            <br />
            <br />
            <SelectedPlayCards />
            <br />
            <br />
            <PlayHand />
            <br />
            <br />
            <Tricks />
          </>
    }
}
