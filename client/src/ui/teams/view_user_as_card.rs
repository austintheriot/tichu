use crate::global::state::AppContext;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ViewUserAsCardProps {
    pub user_id: String,
}

#[function_component(ViewUserAsCard)]
pub fn view_user_as_card(props: &ViewUserAsCardProps) -> Html {
    let app_context = use_context::<AppContext>().expect("AppContext not found");
    let app_state = &*app_context.app_reducer_handle;
    let user = &app_state.game_state.as_ref().map(|game_state| {
        game_state
            .get_user_by_user_id(&props.user_id)
            .expect("Should fin")
    });

    let mut classes = vec!["view-user-as-card".to_string()];
    if props.user_id == app_state.user_id {
        classes.push("selected".into());
    }

    if let Some(user) = &user {
        html! {
          <div class={classes!(classes)}>
            <p>{&user.display_name}</p>
          </div>
        }
    } else {
        html! {}
    }
}
