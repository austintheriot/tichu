use crate::global::state::{AppContext, AppReducerAction};
use yew::prelude::*;

#[function_component(SelectedPlayCards)]
pub fn selected_play_cards() -> Html {
    let app_context = use_context::<AppContext>().expect("AppContext not found");
    let app_state = &*app_context.app_reducer_handle;

    let make_handle_remove_selected_play_card = |i: usize| {
        let reducer_handle = app_context.app_reducer_handle.clone();
        Callback::from(move |_: MouseEvent| {
            reducer_handle.dispatch(AppReducerAction::RemoveSelectedPlayCard(i))
        })
    };

    html! {
      for app_state.selected_play_cards.iter().enumerate().map(|(i, selected_card)| {
        html! {
            <button
                onclick={make_handle_remove_selected_play_card(i)}
                type="button"
              >
                {&format!("Remove {:#?}", selected_card)}
            </button>
        }
    })
    }
}
