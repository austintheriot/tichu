use crate::global::{
    state::{AppContext, AppReducerAction},
    ws::CTSMsgInternal,
};
use crate::ui::common::button::Button;
use crate::ui::common::input::Input;
use crate::ui::common::layout::Layout;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

#[function_component(Join)]
pub fn join() -> Html {
    let app_context = use_context::<AppContext>().expect("AppContext not found");

    let handle_join_room_form_submit = {
        let send_ws_message = app_context.send_ws_message.clone();
        Callback::from(move |e: FocusEvent| {
            e.prevent_default();
            send_ws_message.emit(CTSMsgInternal::JoinGameWithGameCode);
        })
    };

    let handle_join_room_display_name_input = {
        let reducer_handle = app_context.app_reducer_handle.clone();
        Callback::from(move |e: InputEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            let msg = input.map(|input| AppReducerAction::SetDisplayNameInput(input.value()));
            reducer_handle.dispatch(msg.unwrap());
        })
    };

    let handle_join_room_room_code_input = {
        let reducer_handle = app_context.app_reducer_handle.clone();
        Callback::from(move |e: InputEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            let msg = input.map(|input| AppReducerAction::SetJoinRoomGameCodeInput(input.value()));
            reducer_handle.dispatch(msg.unwrap());
        })
    };

    let handle_create_game_form_submit = {
        let send_ws_message = app_context.send_ws_message.clone();
        Callback::from(move |e: FocusEvent| {
            e.prevent_default();
            send_ws_message.emit(CTSMsgInternal::CreateGame);
        })
    };

    let handle_create_room_display_name_input = {
        let reducer_handle = app_context.app_reducer_handle.clone();
        Callback::from(move |e: InputEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            let msg = input.map(|input| AppReducerAction::SetDisplayNameInput(input.value()));
            reducer_handle.dispatch(msg.unwrap());
        })
    };

    let app_state = &*app_context.app_reducer_handle;

    html! {
        <Layout classes={vec!["join-container".into()]}>
            <h1>{"Tichu"}</h1>
            <form onsubmit={handle_join_room_form_submit}>
                <Input
                    label="Display Name"
                    id="join-room-display-name-input"
                    oninput={handle_join_room_display_name_input}
                    input_type="text"
                    value={app_state.display_name_input.clone()}
                />
                <Input
                    label="Game Code"
                    id="join-room-game-code-input"
                    input_type="text"
                    oninput={handle_join_room_room_code_input}
                    value={app_state.join_room_game_code_input.clone()}
                />
                <Button
                    button_type="submit"
                    disabled={!app_state.can_join_game()}
                    >
                    {"Join game"}
                    </Button>
            </form>
            <hr />
            <form onsubmit={handle_create_game_form_submit} >
                <Input
                    label="Display Name"
                    id="join-room-display-name-input"
                    input_type="text"
                    oninput={handle_create_room_display_name_input}
                    value={app_state.display_name_input.clone()}
                />
                <Button
                    button_type="submit"
                    disabled={!app_state.can_create_game()}
                >
                    {"Create game"}
                </Button>
            </form>
        </Layout>
    }
}
