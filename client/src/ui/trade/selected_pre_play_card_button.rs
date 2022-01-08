use crate::global::state::{AppContext, AppReducerAction};
use yew::prelude::*;

#[function_component(SelectedPrePlayCardButton)]
pub fn selected_pre_play_card_button() -> Html {
    let app_context = use_context::<AppContext>().expect("AppContext not found");
    let app_state = &*app_context.app_reducer_handle;
    let handle_remove_selected_card = {
        let reducer_handle = app_context.app_reducer_handle.clone();
        Callback::from(move |_: MouseEvent| {
            reducer_handle.dispatch(AppReducerAction::RemoveSelectedPrePlayCard)
        })
    };

    match &app_state.selected_pre_play_card {
        Some(card) => {
            html! {
                    <button
                        onclick={handle_remove_selected_card}
                        disabled={app_state.selected_pre_play_card.is_none()}
                        type="button">
                        {&format!("Remove {:#?}", card)}
                    </button>
            }
        }
        None => html! {
            <button
                disabled=true
                type="button"
                >
                {"No card selected"}
            </button>
        },
    }
}
