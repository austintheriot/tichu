use crate::global::state::{AppContext, AppReducerAction};
use yew::prelude::*;

#[function_component(ChooseOpponentInput)]
pub fn choose_opponent_input() -> Html {
    let app_context = use_context::<AppContext>().expect("AppContext not found");
    let app_state = &*app_context.app_reducer_handle;
    let opponent_ids = app_state.get_opponent_ids();
    let make_set_use_id_to_give_dragon_to = |id: Option<String>| {
        let reducer_handle = app_context.app_reducer_handle.clone();
        Callback::from(move |_: MouseEvent| {
            reducer_handle.dispatch(AppReducerAction::SetUserIdToGiveDragonTo(id.clone()))
        })
    };

    if let Some(game_state) = &app_state.game_state {
        if let Some((opponent_id_0, opponent_id_1)) = opponent_ids {
            html! {
                <>
                    <p>{"Choose opponent"}</p>
                    // opponent 0
                    {if app_state.user_id_to_give_dragon_to == Some(opponent_id_0.clone()) {
                        html! {
                            <button
                                onclick={make_set_use_id_to_give_dragon_to(None)}
                            >
                                {format!("Deselect {}", &game_state.get_user_by_user_id(&opponent_id_0).unwrap().display_name)}
                            </button>
                        }
                    } else {
                        html!{
                            <button
                                onclick={make_set_use_id_to_give_dragon_to(Some(opponent_id_0.clone()))}
                            >
                                {format!("Select {}", &game_state.get_user_by_user_id(&opponent_id_0).unwrap().display_name)}
                            </button>
                        }
                    }}
                    // opponent 1
                    {if app_state.user_id_to_give_dragon_to == Some(opponent_id_1.clone()) {
                        html! {
                            <button
                                onclick={make_set_use_id_to_give_dragon_to(None)}
                            >
                                {format!("Deselect {}", &game_state.get_user_by_user_id(&opponent_id_1).unwrap().display_name)}
                            </button>
                        }
                    } else {
                        html!{
                            <button
                                onclick={make_set_use_id_to_give_dragon_to(Some(opponent_id_1.clone()))}
                            >
                                {format!("Select {}", &game_state.get_user_by_user_id(&opponent_id_1).unwrap().display_name)}
                            </button>
                        }
                    }}
                </>
            }
        } else {
            html! {}
        }
    } else {
        html! {}
    }
}
