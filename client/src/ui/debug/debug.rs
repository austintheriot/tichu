use super::grand_tichus::AllGrandTichus;
use super::owner::Owner;
use super::skip_to_play_button::SkipToPlay;
use super::small_tichus::AllSmallTichus;
use super::teams::Teams;
use super::user_id_to_give_dragon_to::UserIdToGiveDragonTo;
use super::view_participants::ViewParticipants;
use super::wished_for_card::WishedForCard;
use crate::global::{
    state::{AppContext, WSConnectionStatus},
    ws::CTSMsgInternal,
};
use yew::prelude::*;

#[function_component(Debug)]
pub fn debug() -> Html {
    let app_context = use_context::<AppContext>().expect("no ctx found");
    let app_state = &*app_context.app_reducer_handle;
    let send_test_message = {
        let send_ws_message = app_context.send_ws_message.clone();
        Callback::from(move |_: MouseEvent| {
            send_ws_message.emit(CTSMsgInternal::Test);
        })
    };
    let send_ping_message = {
        let send_ws_message = app_context.send_ws_message.clone();
        Callback::from(move |_: MouseEvent| {
            send_ws_message.emit(CTSMsgInternal::Ping);
        })
    };

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
                <Owner />
                <Teams />
                <UserIdToGiveDragonTo />
                <WishedForCard />
                <h2>{"Grand Tichus: "}</h2>
                <AllGrandTichus />
                <br />
                <h2>{"Small Tichus: "}</h2>
                <AllSmallTichus />
                <button onclick={send_test_message}>{"Send test message to server"}</button>
                <br />
                <button onclick={send_ping_message}>{"Send ping to server"}</button>
          </>
    }
}
