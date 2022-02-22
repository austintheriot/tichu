use crate::global::state::AppContext;
use common::PublicGameStage;
use yew::prelude::*;

#[function_component(Turns)]
pub fn turns() -> Html {
    let app_context = use_context::<AppContext>().expect("AppContext not found");
    let app_state = &*app_context.app_reducer_handle;

    if let Some(game_state) = &app_state.game_state {
        if let PublicGameStage::Play(play_state) = &game_state.stage {
            return html! {
              <div class="turns">
                {for play_state.get_users_in_turn_order().iter().map(|user_id| {
                  let user = game_state.get_user_by_user_id(user_id).expect("Could not find user_id for turn in game state");
                  let mut classes = vec!["display-name".to_string()];
                  if **user_id == play_state.turn_user_id {
                    classes.push("selected".to_string());
                  }
                  html!{
                    <p class={classes}>{&user.display_name}</p>
                  }
                })}
              </div>
            };
        };
    }

    html! {}
}
