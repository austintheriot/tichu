use common::OtherPlayerOption;
use yew::prelude::*;

use crate::global::state::{AppContext, AppReducerAction};

#[derive(Properties, PartialEq)]
pub struct TradeToPersonProps {
    pub player: OtherPlayerOption,
}

#[function_component(TradeToPerson)]
pub fn trade_to_person(props: &TradeToPersonProps) -> Html {
    let app_context = use_context::<AppContext>().expect("AppContext not found");
    let app_state = &*app_context.app_reducer_handle;

    let trade_to_person_state = match &props.player {
        OtherPlayerOption::Opponent1 => &app_state.trade_to_opponent1,
        OtherPlayerOption::Teammate => &app_state.trade_to_teammate,
        OtherPlayerOption::Opponent2 => &app_state.trade_to_opponent2,
    };

    let trade_to_person_display_name = {
        let public_other_players = &app_state
            .game_state
            .as_ref()
            .and_then(|game_state| game_state.get_other_players());

        if let Some(public_other_players) = public_other_players {
            match &props.player {
                OtherPlayerOption::Opponent1 => {
                    public_other_players.opponent_1.display_name.clone()
                }
                OtherPlayerOption::Teammate => public_other_players.teammate.display_name.clone(),
                OtherPlayerOption::Opponent2 => {
                    public_other_players.opponent_2.display_name.clone()
                }
            }
        } else {
            String::new()
        }
    };

    let handle_remove_trade = {
        let reducer_handle = app_context.app_reducer_handle.clone();
        let player = props.player.clone();
        Callback::from(move |_: MouseEvent| {
            reducer_handle.dispatch(AppReducerAction::RemoveTrade(player.clone()))
        })
    };

    let handle_set_trade = {
        let reducer_handle = app_context.app_reducer_handle.clone();
        let player = props.player.clone();
        Callback::from(move |_: MouseEvent| {
            reducer_handle.dispatch(AppReducerAction::SetTrade(player.clone()))
        })
    };

    html! {
          <>
            {if app_state.selected_pre_play_card.is_none() {
                html!{
                    <button
                        disabled={!app_state.can_remove_trade(&props.player)}
                        onclick={handle_remove_trade}
                    >
                    {match trade_to_person_state {
                        Some(card) => format!("Remove {:?} to {}", card, trade_to_person_display_name),
                        None => format!("No trade selected for {}", trade_to_person_display_name),
                    }}
                    </button>
                }
            } else {
                    html!{
                        <button onclick={handle_set_trade}>
                            {if trade_to_person_state.is_some() {
                                format!("Replace trade with {}", trade_to_person_display_name)
                            } else {
                                    format!("Send to {}", trade_to_person_display_name)
                            }}
                        </button>
                }
            }}
          </>
    }
}
