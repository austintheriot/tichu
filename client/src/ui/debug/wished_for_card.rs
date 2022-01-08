use crate::global::state::AppContext;
use yew::prelude::*;

#[function_component(WishedForCard)]
pub fn wished_for_card() -> Html {
    let app_context = use_context::<AppContext>().expect("AppContext not found");
    let app_state = &*app_context.app_reducer_handle;
    let wished_for_card_value = &app_state
        .game_state
        .as_ref()
        .and_then(|game_state| game_state.get_wished_for_card_value());

    html! {
        <>
          <p>{"Wished for Card:"}</p>
          <p>{format!("{:#?}", wished_for_card_value)}</p>
        </>
    }
}
