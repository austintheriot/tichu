use crate::global::state::{AppContext, AppReducerAction};
use yew::prelude::*;

#[function_component(PrePlayHand)]
pub fn pre_play_hand() -> Html {
    let app_context = use_context::<AppContext>().expect("AppContext not found");
    let app_state = &*app_context.app_reducer_handle;
    let make_handle_select_pre_play_card = |i: usize| {
        let reducer_handle = app_context.app_reducer_handle.clone();
        Callback::from(move |_: MouseEvent| {
            reducer_handle.dispatch(AppReducerAction::SetSelectedPrePlayCard(i))
        })
    };

    if let Some(game_state) = &app_state.game_state {
        html! {
                <ul>
                    {for game_state.current_user.hand.iter().enumerate().map(|(i, card)| {
                        // do not render card if the stage is Trade and it is currently selected
                        // OR if it has been selected for trade with opponent
                    let card_is_selected = if let Some(selected_pre_play_card) = &app_state.selected_pre_play_card {
                        *selected_pre_play_card == *card
                    } else {
                            false
                    };
                    if (card_is_selected || app_state.is_card_is_set_to_trade(card)) && app_state.stage_is_trade() {
                        html!{}
                    } else {
                            html!{
                                <li>
                                    <button
                                        disabled={!app_state.can_select_pre_play_card()}
                                        onclick={make_handle_select_pre_play_card(i)}
                                        >
                                        {&format!("{:#?}", card)}
                                    </button>
                                </li>
                        }
                    }
                })}
                </ul>
        }
    } else {
        html! {}
    }
}
