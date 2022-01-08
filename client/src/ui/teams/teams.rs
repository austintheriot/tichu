use super::team_button::TeamButton;
use super::team_input::TeamInput;
use crate::global::{state::AppContext, ws::CTSMsgInternal};
use common::TeamOption;
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

    html! {
          <>
              <h1>{"Teams"}</h1>
              <TeamInput
                team_option={TeamOption::TeamA}
                title={"Team Name".to_string()}
                input_label_id={"team-a-name-input".to_string()}
              />
              <br />
              <TeamButton text={"Move to Team A".to_string()} team_option={TeamOption::TeamA} />
              <br />
              <TeamButton text={"Move to Team B".to_string()} team_option={TeamOption::TeamB} />
              <br />
             <TeamInput
                team_option={TeamOption::TeamB}
                title={"Team Name".to_string()}
                input_label_id={"team-b-name-input".to_string()}
              />
             {if app_state.is_current_user_owner() {
                html!{
                  <button
                      onclick={handle_start_grand_tichu}
                      disabled={!app_state.can_start_game()}
                  >
                    {"Start"}
                  </button>
              }
          } else {
                 html!{}
          }}
          </>
    }
}
