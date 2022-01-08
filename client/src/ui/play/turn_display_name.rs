use crate::global::state::AppContext;
use common::PublicGameStage;
use yew::prelude::*;

#[function_component(TurnDisplayName)]
pub fn turn_display_name() -> Html {
    let app_context = use_context::<AppContext>().expect("AppContext not found");
    let app_state = &*app_context.app_reducer_handle;

    if let Some(game_state) = &app_state.game_state {
        if let PublicGameStage::Play(play_state) = &game_state.stage {
            let turn_user_id = &play_state.turn_user_id;
            let turn_user = &game_state.get_user_by_user_id(turn_user_id);
            if let Some(turn_user) = turn_user {
                return html! {
                    <p>{&format!("Current turn: {}", turn_user.display_name)}</p>
                };
            }
        }
    }

    html! {}
}
