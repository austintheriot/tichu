use yew::prelude::*;

use crate::global::state::AppContext;

#[function_component(ViewParticipants)]
pub fn view_participants() -> Html {
    let app_context = use_context::<AppContext>().expect("AppContext not found");
    let app_state = &*app_context.app_reducer_handle;

    if let Some(game_state) = &app_state.game_state {
        html! {
            <ul>
                {for game_state.participants.iter().map(|user| {
                    html! {
                        <li>{&user.display_name}</li>
                    }
                })}
            </ul>
        }
    } else {
        html! {}
    }
}
