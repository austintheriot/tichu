use crate::global::{state::AppContext, ws::CTSMsgInternal};
use common::TeamOption;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TeamButtonProps {
    pub text: String,
    pub team_option: TeamOption,
}

#[function_component(TeamButton)]
pub fn team_input(props: &TeamButtonProps) -> Html {
    let app_context = use_context::<AppContext>().expect("AppContext not found");

    let handle_move_to_team = {
        let send_ws_message = app_context.send_ws_message.clone();
        let team_option = props.team_option.clone();
        Callback::from(move |_: MouseEvent| {
            send_ws_message.emit(CTSMsgInternal::MoveToTeam(team_option.clone()));
        })
    };

    let app_state = &*app_context.app_reducer_handle;

    let is_on_team_already = match &props.team_option {
        TeamOption::TeamA => app_state.is_on_team_a(),
        TeamOption::TeamB => app_state.is_on_team_b(),
    };

    html! {
        <button
            onclick={handle_move_to_team}
            disabled={!app_state.is_team_stage() || is_on_team_already}
        >
            {props.text.clone()}
        </button>
    }
}
