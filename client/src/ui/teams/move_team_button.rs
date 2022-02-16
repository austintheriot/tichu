use crate::global::{state::AppContext, ws::CTSMsgInternal};
use crate::ui::common::button::{Button, ButtonVariant};
use crate::ui::icons::up_arrow::UpArrow;
use common::TeamOption;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MoveTeamButtonProps {
    pub team_option: TeamOption,
}

#[function_component(MoveTeamButton)]
pub fn team_input(props: &MoveTeamButtonProps) -> Html {
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

    let mut classes = vec!["move-team-button".to_string()];
    if props.team_option == TeamOption::TeamB {
        classes.push("rotate".into());
    };

    html! {
        <Button
            onclick={handle_move_to_team}
            disabled={!app_state.is_team_stage() || is_on_team_already}
            variant={ButtonVariant::Circle}
            {classes}
        >
            <UpArrow />
        </Button>
    }
}
