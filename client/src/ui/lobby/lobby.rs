use super::participants_list::ParticipantsList;
use crate::global::{state::AppContext, ws::CTSMsgInternal};
use crate::ui::common::button::{Button, ButtonVariant};
use crate::ui::common::layout::Layout;
use crate::ui::icons::check::Check;
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

    return if let Some(game_state) = &app_state.game_state {
        html! {
                <Layout classes={vec!["lobby-container".into()]}>
                    <p class="game-code-title">
                        {"Game Code: "}
                    </p>
                    <p class="game-code-string">
                        {&game_state.game_code}
                    </p>

                    <Button
                        variant={ButtonVariant::Circle}
                        disabled=true
                    >
                        <Check style="stroke: var(--gray-75);" />
                    </Button>

                    <p class="joined">{"Joined:"}</p>
                    <ParticipantsList />

                    <Button
                        onclick={send_leave_game_message}
                        disabled={!app_state.can_leave_game()}
                    >
                        {"Leave"}
                    </Button>
                </Layout>
        }
    } else {
        html! {}
    };
}
