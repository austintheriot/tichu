use yew::prelude::*;

use crate::app::state::{AppContext, AppReducerAction};

#[function_component(Lobby)]
pub fn lobby() -> Html {
    let parent_context = use_context::<AppContext>().expect("no ctx found");
    let onclick = {
        let reducer_handle = parent_context.reducer_handle.clone();
        Callback::from(move |_: MouseEvent| reducer_handle.dispatch(AppReducerAction::Debug))
    };

    html! {
        <button {onclick}>
            {"Pass Debug Action"}
        </button>
    }
}
