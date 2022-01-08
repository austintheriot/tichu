use crate::global::state::AppContext;
use common::{PublicGameStage, TichuCallStatus};
use yew::prelude::*;

#[function_component(GrandTichuStatusCurrentUser)]
pub fn grand_tichu_status_current_user() -> Html {
    let app_context = use_context::<AppContext>().expect("AppContext not found");
    let app_state = &*app_context.app_reducer_handle;

    let grand_tichu_call_status = match &app_state.game_state {
        Some(game_state) => match &game_state.stage {
            PublicGameStage::GrandTichu(grand_tichu_state) => {
                match grand_tichu_state
                    .grand_tichus
                    .iter()
                    .find(|user_id_with_tichu_call_status| {
                        *user_id_with_tichu_call_status.user_id == *app_state.user_id
                    }) {
                    Some(user_id_with_tichu_call_status) => {
                        match user_id_with_tichu_call_status.tichu_call_status {
                            TichuCallStatus::Undecided => "Undecided",
                            TichuCallStatus::Called => "Called",
                            TichuCallStatus::Declined => "Declined",
                            TichuCallStatus::Achieved => "Achieved",
                            TichuCallStatus::Failed => "Failed",
                        }
                    }
                    None => "n/a",
                }
            }
            _ => "n/a",
        },
        None => "n/a",
    };
    html! {
            <p>{"Grand Tichu Call Status: "} {grand_tichu_call_status} {"\n"}</p>
    }
}
