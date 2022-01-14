use crate::global::state::AppContext;
use common::PublicGameStage;
use yew::prelude::*;

#[function_component(Scores)]
pub fn scores() -> Html {
    let app_context = use_context::<AppContext>().expect("AppContext not found");
    let app_state = &*app_context.app_reducer_handle;

    if let Some(game_state) = &app_state.game_state {
        if let PublicGameStage::Play(play_state) = &game_state.stage {
            return html! {
              <>
                <p>{"Scores:"}</p>
                {for play_state.teams.iter().map(|team| {
                  html!{
                   <>
                    <p>{&team.team_name}</p>
                    <p>{&team.score}</p>
                   </>
                  }
                })}
              </>
            };
        };
    }

    html! {}
}
