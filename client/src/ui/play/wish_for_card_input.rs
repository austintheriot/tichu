use crate::global::state::{AppContext, AppReducerAction};
use common::Deck;
use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

#[function_component(WishForCardInput)]
pub fn wish_for_card_input() -> Html {
    let app_context = use_context::<AppContext>().expect("AppContext not found");
    let handle_wished_for_card_input = {
        let reducer_handle = app_context.app_reducer_handle.clone();
        Callback::from(move |e: InputEvent| {
            let target = e.target();
            let select = target.and_then(|t| t.dyn_into::<HtmlSelectElement>().ok());
            let msg = select
                .map(|select| AppReducerAction::SetWishedForCard(select.selected_index() as usize))
                .unwrap();
            reducer_handle.dispatch(msg);
        })
    };

    html! {
        <>
            <label for="wish-for-card">{"Wish for a card?"}</label>
            <select name="wish-for-card" id="wish-for-card"
                oninput={handle_wished_for_card_input}
            >
                {for Deck::wished_for_card_values().iter().enumerate().map(|(i, card)| {
                    let card_string = format!("{:#?}", card);
                    html!{
                        <option value={format!("{}", i)}>
                            {card_string.clone()}
                        </option>
                    }
                })}
            </select>
        </>
    }
}