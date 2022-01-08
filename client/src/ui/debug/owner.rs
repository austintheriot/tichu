use yew::prelude::*;

use crate::global::state::AppContext;

#[function_component(Owner)]
pub fn owner() -> Html {
    let app_context = use_context::<AppContext>().expect("AppContext not found");
    let app_state = &*app_context.app_reducer_handle;

    if let Some(game_state) = &app_state.game_state {
        let owner = game_state
            .participants
            .iter()
            .find(|user| user.user_id == game_state.owner_id);

        match owner {
            Some(owner) => {
                html! {
                    <ul>
                        <li>{&owner.display_name}</li>
                    </ul>
                }
            }
            None => {
                html! {}
            }
        }
    } else {
        html! {}
    }
}
