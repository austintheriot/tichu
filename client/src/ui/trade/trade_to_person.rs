use crate::global::state::{AppContext, AppReducerAction};
use crate::ui::common::button::Button;
use crate::ui::icons::add::Add;
use common::OtherPlayerOption;
use yew::prelude::*;

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

    let trade_to_person = {
        let public_other_players = &app_state
            .game_state
            .as_ref()
            .and_then(|game_state| game_state.get_other_players());

        if let Some(public_other_players) = public_other_players {
            match &props.player {
                OtherPlayerOption::Opponent1 => Some(public_other_players.opponent_1.clone()),
                OtherPlayerOption::Teammate => Some(public_other_players.teammate.clone()),
                OtherPlayerOption::Opponent2 => Some(public_other_players.opponent_2.clone()),
            }
        } else {
            None
        }
    }
    .expect("Couldn't find trade to person in trade_to_person.rs");

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

    let onclick = if app_state.selected_pre_play_card.is_none() {
        handle_remove_trade
    } else {
        handle_set_trade
    };

    let disabled =
        app_state.selected_pre_play_card.is_none() && !app_state.can_remove_trade(&props.player);

    html! {
        <Button
            classes={vec!["trade-to-person-button".to_string()]}
            {disabled}
            {onclick}
        >
            <p class="display-name">{trade_to_person.display_name.clone()}</p>
            {match trade_to_person_state {
                // card has been selected for person
                Some(card) => html!{
                   <div class="card-icon-container">
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
                   </div>
                },
                // card has NOT been selected for person
                None => html!{
                    <div class="add-container">
                        <Add />
                    </div>
                },
            }}
        </Button>
    }
}
