use crate::global::state::AppContext;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MutableTeamProps {
    pub team: common::MutableTeam,
}

#[function_component(MutableTeam)]
pub fn mutable_team(props: &MutableTeamProps) -> Html {
    let app_context = use_context::<AppContext>().expect("AppContext not found");
    let app_state = &*app_context.app_reducer_handle;

    html! {
          <ul>
              <li>{"Team Name: "} {{&props.team.team_name}}</li>
              <li>{"Score: "} {{&props.team.score}}</li>
              <li>{"Users: "} {for props.team.user_ids.iter().map(|id| {
                    html!{
                       <p>{match &app_state.game_state
                            .as_ref()
                            .and_then(|game_state| game_state.get_user_by_user_id(id)) {
                            Some(participant) => &participant.display_name,
                            None => ""
                              }
                          }
                       </p>
                    }
                })}
              </li>
          </ul>
    }
}
