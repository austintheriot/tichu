use crate::global::{
    state::{AppContext, AppReducerAction},
    ws::CTSMsgInternal,
};
use crate::ui::common::input::Input;
use common::TeamOption;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TeamInputProps {
    pub title: String,
    pub input_label_id: String,
    pub team_option: TeamOption,
}

#[function_component(TeamInput)]
pub fn team_input(props: &TeamInputProps) -> Html {
    let app_context = use_context::<AppContext>().expect("AppContext not found");

    let handle_team_submit = {
        let send_ws_message = app_context.send_ws_message.clone();
        let team_option = props.team_option.clone();
        Callback::from(move |e: FocusEvent| {
            e.prevent_default();
            send_ws_message.emit(CTSMsgInternal::RenameTeam(team_option.clone()));
        })
    };

    let handle_team_input = {
        let reducer_handle = app_context.app_reducer_handle.clone();
        let team_option = props.team_option.clone();
        Callback::from(move |e: InputEvent| {
            let target = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            let msg = input
                .map(|input| match team_option {
                    TeamOption::TeamA => AppReducerAction::SetTeamANameInput(input.value()),
                    TeamOption::TeamB => AppReducerAction::SetTeamBNameInput(input.value()),
                })
                .unwrap();
            reducer_handle.dispatch(msg);
        })
    };

    let app_state = &*app_context.app_reducer_handle;

    let is_on_opposite_team = match &props.team_option {
        TeamOption::TeamA => app_state.is_on_team_b(),
        TeamOption::TeamB => app_state.is_on_team_a(),
    };

    let input_value = match &props.team_option {
        TeamOption::TeamA => app_state.team_a_name_input.clone(),
        TeamOption::TeamB => app_state.team_b_name_input.clone(),
    };

    html! {
        <form onsubmit={handle_team_submit}>
            <Input
                label="Team Name"
                id={props.input_label_id.clone()}
                oninput={handle_team_input}
                input_type="text"
                value={input_value}
                disabled={!app_state.is_team_stage() || is_on_opposite_team}
            />
        </form>
    }
}
