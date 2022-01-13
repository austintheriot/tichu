use crate::global::state::AppContext;
use common::PublicGameStage;
use yew::prelude::*;

#[function_component(WishedForCard)]
pub fn wished_for_card() -> Html {
    let app_context = use_context::<AppContext>().expect("AppContext not found");
    let app_state = &*app_context.app_reducer_handle;

    if let Some(game_state) = &app_state.game_state {
        if let PublicGameStage::Play(play_state) = &game_state.stage {
            if let Some(wished_for_card_value) = &play_state.wished_for_card_value {
                return html! {
                   <>
                    <p>{"Wished for card value:"}</p>
                    <p>{format!("{:#?}", wished_for_card_value)}</p>
                   </>
                };
            }
        }
    }

    html! {}
}
