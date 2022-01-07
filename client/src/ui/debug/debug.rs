use yew::prelude::*;

use super::skip_to_play_button::SkipToPlay;
use super::view_participants::ViewParticipants;
use crate::global::{
    state::{AppContext, AppReducerAction, WSConnectionStatus},
    ws::CTSMsgInternal,
};

#[function_component(Debug)]
pub fn debug() -> Html {
    let app_context = use_context::<AppContext>().expect("no ctx found");
    let handle_debug = {
        let reducer_handle = app_context.app_reducer_handle.clone();
        Callback::from(move |_: MouseEvent| reducer_handle.dispatch(AppReducerAction::Debug))
    };
    let handle_ping = {
        let send_ws_message = app_context.send_ws_message.clone();
        Callback::from(move |_: MouseEvent| {
            send_ws_message.emit(CTSMsgInternal::Ping);
        })
    };

    let app_state = &*app_context.app_reducer_handle;

    html! {
          <>
          <h1>{"Debug Info:"}</h1>
              <p>{"Display Name: "} {&app_state.display_name}</p>
              <p>{"User ID: "} {&app_state.user_id}</p>
              <p>{"Websocket Status: "}{match &app_state.ws_connection_status {
                  WSConnectionStatus::Open => "Connected",
                  WSConnectionStatus::Closed => "Not Connected",
                  WSConnectionStatus::Error => "Error"
          }}</p>
              <p>{"Game Code: "} {
                  if let Some(game_state) = &app_state.game_state {
                      &game_state.game_code
              } else {
                      ""
              }}
              </p>
              <SkipToPlay />
              <ViewParticipants />
              // <p>{"Participants: "} {self.view_participants()}</p>
              // <p>{"Owner: "} {self.debug_owner()}</p>
              // <p>{"Teams: "} {self.debug_teams()}</p>
              // {self.debug_user_id_to_give_dragon_to()}
              // {self.debug_wished_for_card()}
              // <h2>{"Grand Tichus: "}</h2>
              // {self.view_debug_all_participants_grand_tichu()}
              // <br />
              // <h2>{"Small Tichus: "}</h2>
              // {self.view_debug_all_participants_small_tichu()}
              // <button onclick={link.callback(|_| AppMsg::SendWSMsg(CTSMsgInternal::Test))}>{"Send test message to server"}</button>
              // <br />
              // <button onclick={link.callback(|_| AppMsg::SendWSMsg(CTSMsgInternal::Ping))}>{"Send ping to server"}</button>
          </>
    }
}
