use crate::global::state::{AppContext, AppReducerAction};
use common::{PublicGameStage, PublicGameState};
use yew::prelude::*;

#[function_component(PrePlayHand)]
pub fn pre_play_hand() -> Html {
    let app_context = use_context::<AppContext>().expect("AppContext not found");
    let app_state = &*app_context.app_reducer_handle;
    let make_handle_select_pre_play_card = |i: usize, disabled: bool| {
        let reducer_handle = app_context.app_reducer_handle.clone();
        Callback::from(move |_: MouseEvent| {
            if disabled {
                return;
            }
            reducer_handle.dispatch(AppReducerAction::SetSelectedPrePlayCard(i))
        })
    };
    let handle_remove_selected_card = {
        let reducer_handle = app_context.app_reducer_handle.clone();
        Callback::from(move |_: MouseEvent| {
            reducer_handle.dispatch(AppReducerAction::RemoveSelectedPrePlayCard)
        })
    };

    // disable card when user is looking at them to try and decide whether to call grand tichu or not
    let all_cards_disabled = if let Some(PublicGameState {
        stage: PublicGameStage::GrandTichu(_),
        ..
    }) = &app_state.game_state
    {
        true
    } else {
        false
    };

    if let Some(game_state) = &app_state.game_state {
        html! {
                <ul class="pre-play-hand">
                    {for game_state.current_user.hand.iter().enumerate().map(|(i, card)| {

                    // card has been selected but hasn't yet been set to trade: highlight the card
                    let card_is_selected = if let Some(selected_pre_play_card) = &app_state.selected_pre_play_card {
                        *selected_pre_play_card == *card
                    } else {
                            false
                    };

                    let class = if card_is_selected {
                        "selected"
                    } else {
                        ""
                    };

                    let disabled = !app_state.can_select_pre_play_card() || all_cards_disabled;

                    let handle_click = if card_is_selected {
                        handle_remove_selected_card.clone()
                    } else {
                        make_handle_select_pre_play_card(i, disabled)
                    };


                    // do not render card if the stage is Trade and it has been selected for trade with opponent
                    if app_state.stage_is_trade() && app_state.is_card_is_set_to_trade(card) {
                        html!{}
                    } else {
                            html!{
                                <li>
                                    <button
                                        onclick={handle_click}
                                        {disabled}
                                        {class}
                                    >
                                        if card.suit.is_special() {
                                            <span class="special">{card.suit.icon()}</span>
                                        } else {
                                            <span class="regular-value">
                                                {card.value.icon()}
                                            </span>
                                            <span class="regular-suit">
                                                {card.suit.icon()}
                                            </span>
                                        }
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
