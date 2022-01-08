use super::immutable_team::ImmutableTeam;
use super::mutable_team::MutableTeam;
use crate::global::state::AppContext;
use common::PublicGameStage;
use yew::prelude::*;

#[function_component(Teams)]
pub fn teams() -> Html {
    let app_context = use_context::<AppContext>().expect("AppContext not found");
    let app_state = &*app_context.app_reducer_handle;

    if let Some(game_state) = &app_state.game_state {
        match &game_state.stage {
            PublicGameStage::Teams(team_state) => {
                html! {
                    <ul>
                        <li><MutableTeam team={team_state[0].clone()} /></li>
                        <li><MutableTeam team={team_state[1].clone()} /></li>
                    </ul>
                }
            }
            PublicGameStage::GrandTichu(grand_tichu_state) => {
                html! {
                    <ul>
                        <li><ImmutableTeam team={grand_tichu_state.teams[0].clone()} /></li>
                        <li><ImmutableTeam team={grand_tichu_state.teams[1].clone()} /></li>
                    </ul>
                }
            }
            PublicGameStage::Trade(trade_state) => {
                html! {
                    <ul>
                        <li><ImmutableTeam team={trade_state.teams[0].clone()} /></li>
                        <li><ImmutableTeam team={trade_state.teams[1].clone()} /></li>
                    </ul>
                }
            }
            PublicGameStage::Play(play_state) => {
                html! {
                    <ul>
                        <li><ImmutableTeam team={play_state.teams[0].clone()} /></li>
                        <li><ImmutableTeam team={play_state.teams[1].clone()} /></li>
                    </ul>
                }
            }
            _ => html! {},
        }
    } else {
        html! {}
    }
}
