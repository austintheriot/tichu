use crate::global::state::AppContext;
use yew::prelude::*;

#[function_component(Tricks)]
pub fn tricks() -> Html {
    let app_context = use_context::<AppContext>().expect("AppContext not found");
    let app_state = &*app_context.app_reducer_handle;

    if let Some(game_state) = &app_state.game_state {
        html! {
          <>
            {for game_state.current_user.tricks.iter().map(|trick| {
                html! {
                  <p>{format!("{:#?}", trick)}</p>
                }
            })}
          </>
        }
    } else {
        html! {}
    }
}
