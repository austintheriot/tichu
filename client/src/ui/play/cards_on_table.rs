use crate::global::state::AppContext;
use common::PublicGameStage;
use yew::prelude::*;

#[function_component(CardsOnTable)]
pub fn cards_on_table() -> Html {
    let app_context = use_context::<AppContext>().expect("AppContext not found");
    let app_state = &*app_context.app_reducer_handle;

    if let Some(game_state) = &app_state.game_state {
        if let PublicGameStage::Play(play_state) = &game_state.stage {
            let last_combo = play_state.table.last();
            return if let Some(last_combo) = last_combo {
                html! {
                        <>
                            <p>{"Cards on table:"}</p>
                            <ul>
                                {for last_combo.cards().iter().map(|card| {
                                    html!{
                                        <li>
                                            <p>
                                                {&format!("{}", card)}
                                            </p>
                                        </li>
                                }
                            })}
                            </ul>
                       </>
                }
            } else {
                html! {  <p>{"No cards on table yet"}</p> }
            };
        }
    }

    html! {}
}
