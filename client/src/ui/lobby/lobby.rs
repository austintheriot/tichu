use super::super::debug::view_participants::ViewParticipants;
use crate::global::{state::AppContext, ws::CTSMsgInternal};
use yew::prelude::*;

#[function_component(Lobby)]
pub fn lobby() -> Html {
    let app_context = use_context::<AppContext>().expect("no ctx found");
    let send_leave_game_message = {
        let send_ws_message = app_context.send_ws_message.clone();
        Callback::from(move |_: MouseEvent| {
            send_ws_message.emit(CTSMsgInternal::LeaveGame);
        })
    };
    let app_state = &*app_context.app_reducer_handle;

    html! {
            <>
                <h1>{"Lobby"}</h1>
                <h2>{"Game Code: "} {
                    if let Some(game_state) = &app_state.game_state {
                        &game_state.game_code
                } else {
                        ""
                }
            }
                </h2>
                <h3>{"Joined:"}</h3>
                <br />
                <ViewParticipants />
                <button
                    onclick={send_leave_game_message}
                    disabled={!app_state.can_leave_game()}
                    >
                    {"Leave game"}
                </button>
            </>
    }
}
