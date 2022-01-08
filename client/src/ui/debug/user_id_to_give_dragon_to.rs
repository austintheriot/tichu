use crate::global::state::AppContext;
use yew::prelude::*;

#[function_component(UserIdToGiveDragonTo)]
pub fn user_id_to_give_dragon_to() -> Html {
    let app_context = use_context::<AppContext>().expect("AppContext not found");
    let app_state = &*app_context.app_reducer_handle;

    let user_id_to_give_dragon_to = &app_state.user_id_to_give_dragon_to;
    html! {
        <>
          <p>{"User To Give Dragon To:"}</p>
          <p>{format!("{:#?}", &user_id_to_give_dragon_to)}</p>
          {if let Some(game_state) = &app_state.game_state {
            if let Some(user_id_to_give_dragon_to) = user_id_to_give_dragon_to {
                html!{
                    <p>{format!("{:#?}", game_state.get_user_by_user_id(user_id_to_give_dragon_to).unwrap().display_name)}</p>
                }
            } else {
                html!{}
            }
          } else {
              html!{}
          }}
        </>
    }
}
