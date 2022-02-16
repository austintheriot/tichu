use super::move_team_button::MoveTeamButton;
use super::team_name_input::TeamNameInput;
use super::view_team_as_cards::ViewTeamAsCards;
use crate::global::{state::AppContext, ws::CTSMsgInternal};
use crate::ui::common::button::Button;
use crate::ui::common::layout::Layout;
use common::{PublicGameStage, PublicGameState, TeamOption};
use yew::prelude::*;

#[function_component(Teams)]
pub fn teams() -> Html {
    let app_context = use_context::<AppContext>().expect("AppContext not found");

    let handle_start_grand_tichu = {
        let send_ws_message = app_context.send_ws_message.clone();
        Callback::from(move |_: MouseEvent| {
            send_ws_message.emit(CTSMsgInternal::StartGrandTichu);
        })
    };

    let app_state = &*app_context.app_reducer_handle;

    if let Some(PublicGameState {
        stage: PublicGameStage::Teams(teams_state),
        ..
    }) = &app_state.game_state.as_ref()
    {
        html! {
              <Layout classes={vec!["teams-container".into()]}>
                <h1>{"Teams"}</h1>

                // Team A input
                <div class="input-wrapper">
                  <TeamNameInput
                    team_option={TeamOption::TeamA}
                    title={"Team Name".to_string()}
                    input_label_id={"team-a-name-input".to_string()}
                  />
                  <ViewTeamAsCards team={teams_state[0].clone()} />
                </div>

                // move to Team A
                <MoveTeamButton team_option={TeamOption::TeamA} />

                <hr />

                // move to Team B
                <MoveTeamButton team_option={TeamOption::TeamB} />

                // Team B input
                <div class="input-wrapper">
                  <ViewTeamAsCards team={teams_state[1].clone()} />
                  <TeamNameInput
                      team_option={TeamOption::TeamB}
                      title={"Team Name".to_string()}
                      input_label_id={"team-b-name-input".to_string()}
                  />
                </div>

                // Start button
                {if app_state.is_current_user_owner() {
                  html!{
                    <Button
                        onclick={handle_start_grand_tichu}
                        disabled={!app_state.can_start_game()}
                    >
                      {"Start"}
                    </Button>
                }
              } else {
                     html!{}
              }}
              </Layout>
        }
    } else {
        html! {}
    }
}
