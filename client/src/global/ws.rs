//! Websocket logic for communicating with Tichu server
//!
//! To avoid trying to send on a non-ready websocket, ping_interval is only
//! initialized once the websocket as actually been opened. Occasionally, however,
//! the process doesn't get far enough for ping_interval to get setup.
//!
//! If no ping_interval has been schedule yet, then a manual retry_timeout is scheduled.
//!
//! However, if a ping_interval HAS been scheduled (i.e. the websocket got setup and then it closed),
//! then the ping will automatically try to reopen the websocket if it is closed/not working,
//! so no need to schedule a manual retry timeout.

use crate::global::{
    js_functions::js_log_with_styling,
    state::{AppReducerAction, AppState},
};
use anyhow::Error;
use common::{
    sort_cards_for_hand, validate_team_name, CTSMsg, CallGrandTichuRequest, CardTrade, CardValue,
    OtherPlayerOption, PublicGameStage, STCMsg, TeamOption,
};
use gloo::{
    timers::callback::{Interval, Timeout},
    utils::format::JsValueSerdeExt,
};
use log::*;
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{ErrorEvent, MessageEvent, WebSocket};
use yew::{use_effect_with_deps, use_mut_ref, Callback, UseReducerHandle};

pub const PING_INTERVAL_MS: u32 = 5000;

pub struct WSCallbacks {
    #[allow(dead_code)]
    onmessage: Closure<dyn FnMut(MessageEvent)>,
    #[allow(dead_code)]
    onopen: Closure<dyn FnMut()>,
    #[allow(dead_code)]
    onerror: Closure<dyn FnMut(ErrorEvent)>,
    #[allow(dead_code)]
    onclose: Closure<dyn FnMut()>,
}

#[derive(Default)]
pub struct WSState {
    ws: Option<WebSocket>,
    is_alive: bool,
    ws_callbacks: Option<WSCallbacks>,
    ping_interval: Option<Interval>,
    reschedule_timeout: Option<Timeout>,
}

/// Connects to server websocket and assigns listeners for all websocket events.
/// Once the `onopen` event has been received from the websocket, the websocket begins pinging the server.
pub fn connect_to_ws(
    app_reducer_handle: UseReducerHandle<AppState>,
    ws_mut_ref: Rc<RefCell<WSState>>,
) {
    let ws_is_none = (*ws_mut_ref).borrow().ws.is_none();
    if ws_is_none {
        info!("Opening websocket...");
        let url = format!(
            "ws://localhost:8080/ws?user_id={}",
            (*app_reducer_handle).user_id
        );
        let ws = WebSocket::new(&url);
        let ws = match ws {
            Err(e) => {
                // log and retry on failure
                error!(
                    "Error opening WebSocket connection. Will retry in {} seconds. Error: {:?}",
                    PING_INTERVAL_MS, e
                );
                schedule_ws_reconnect(
                    app_reducer_handle.clone(),
                    ws_mut_ref.clone(),
                    PING_INTERVAL_MS,
                );
                return;
            }
            Ok(ws) => ws,
        };

        ws.set_binary_type(web_sys::BinaryType::Arraybuffer);
        let ws_mut_ref_clone = ws_mut_ref.clone();
        // on message
        let onmessage_callback = {
            let app_reducer_handle = app_reducer_handle.clone();
            let ws_mut_ref = ws_mut_ref_clone.clone();
            Closure::wrap(Box::new(move |e: MessageEvent| {
                // ArrayBuffer
                if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
                    let u_int_8_array = js_sys::Uint8Array::new(&abuf);
                    let vec = u_int_8_array.to_vec();
                    handle_ws_message_received(
                        app_reducer_handle.clone(),
                        ws_mut_ref.clone(),
                        Ok(vec),
                    );
                } else if let Ok(blob) = e.data().dyn_into::<web_sys::Blob>() {
                    // Blob
                    warn!("Websocket message event, received blob: {:?}", blob);
                } else if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                    // Text
                    warn!("Websocket message event, received Text: {:?}", txt);
                } else {
                    // Unknown
                    warn!("Websocket message event, received Unknown: {:?}", e.data());
                }
            }) as Box<dyn FnMut(MessageEvent)>)
        };
        ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));

        // on open
        let ws_mut_ref_clone = ws_mut_ref.clone();
        let onopen_callback = {
            let app_reducer_handle = app_reducer_handle.clone();
            let ws_mut_ref = ws_mut_ref_clone.clone();

            Closure::wrap(Box::new(move || {
                info!("Websocket open event");
                app_reducer_handle.dispatch(AppReducerAction::WebsocketOpen);
                begin_ping(app_reducer_handle.clone(), ws_mut_ref.clone());
            }) as Box<dyn FnMut()>)
        };
        ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));

        // on error
        let onerror_callback = {
            let app_reducer_handle = app_reducer_handle.clone();
            let ws_mut_ref = ws_mut_ref.clone();

            Closure::wrap(Box::new(move |e: ErrorEvent| {
                error!("Websocket error event: {:?}", e);
                app_reducer_handle.dispatch(AppReducerAction::WebsocketError);
                cleanup_ws_state(ws_mut_ref.clone());
                // if ping has been scheduled, then we don't need to retry connection
                if (*ws_mut_ref.borrow()).ping_interval.is_none() {
                    schedule_ws_reconnect(
                        app_reducer_handle.clone(),
                        ws_mut_ref.clone(),
                        PING_INTERVAL_MS,
                    );
                }
            }) as Box<dyn FnMut(ErrorEvent)>)
        };
        ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));

        // on close
        let onclose_callback = {
            let app_reducer_handle = app_reducer_handle.clone();
            let ws_mut_ref = ws_mut_ref.clone();

            Closure::wrap(Box::new(move || {
                error!("Websocket close event");
                app_reducer_handle.dispatch(AppReducerAction::WebsocketClosed);
                cleanup_ws_state(ws_mut_ref.clone());
                // if ping has been scheduled, then we don't need to retry connection
                if (*ws_mut_ref.borrow()).ping_interval.is_none() {
                    schedule_ws_reconnect(
                        app_reducer_handle.clone(),
                        ws_mut_ref.clone(),
                        PING_INTERVAL_MS,
                    );
                }
            }) as Box<dyn FnMut()>)
        };
        ws.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));

        let mut ws_state = (*ws_mut_ref).borrow_mut();
        ws_state.ws = Some(ws);
        ws_state.ws_callbacks = Some(WSCallbacks {
            onmessage: onmessage_callback,
            onopen: onopen_callback,
            onerror: onerror_callback,
            onclose: onclose_callback,
        });
    } else {
        error!("Trying to ConnectToWS while current websocket is still defined as Some() in state");
    }
}

pub fn begin_ping(
    app_reducer_handle: UseReducerHandle<AppState>,
    ws_mut_ref: Rc<RefCell<WSState>>,
) {
    info!("Scheduling WebSocket ping");
    // start pinging on an interval
    let interval = {
        let ws_mut_ref = ws_mut_ref.clone();
        Interval::new(PING_INTERVAL_MS, move || {
            send_ws_message(
                app_reducer_handle.clone(),
                ws_mut_ref.clone(),
                CTSMsgInternal::Ping,
            );
        })
    };
    // we don't need to retry on a timeout if ping is running
    let mut ws_state = (*ws_mut_ref).borrow_mut();
    let timeout = ws_state.reschedule_timeout.take();
    if let Some(timeout) = timeout {
        timeout.cancel();
    }

    ws_state.ping_interval = Some(interval);
}

/// Internal Tichu-client message for alerting that it's time to send a websocket message
///
/// This type reflects the common::CTSMsg, except with all data values tripped,
/// since the data values are formulated in the send_ws_message function
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum CTSMsgInternal {
    JoinGameWithGameCode,
    MoveToTeam(TeamOption),
    RenameTeam(TeamOption),
    CreateGame,
    LeaveGame,
    StartGrandTichu,
    SubmitTrade,
    PlayCards,
    Pass,
    CallGrandTichu(CallGrandTichuRequest),
    CallSmallTichu,
    __AdminSkipToPlay,

    Ping,
    Pong,
    Test,
}

fn schedule_ws_reconnect(
    app_reducer_handle: UseReducerHandle<AppState>,
    ws_mut_ref: Rc<RefCell<WSState>>,
    interval: u32,
) {
    if (*ws_mut_ref).borrow().reschedule_timeout.is_some() {
        warn!("Timeout already scheduled to reopen websocket. Ignoring request.");
        return;
    }
    info!("Will try to reopen websocket in {}ms", interval);
    let new_timeout = {
        let ws_mut_ref = ws_mut_ref.clone();
        Timeout::new(interval, move || {
            // delete its handle in state
            let timeout_in_state = (*ws_mut_ref).borrow_mut().reschedule_timeout.take();
            drop(timeout_in_state);
            connect_to_ws(app_reducer_handle.clone(), ws_mut_ref.clone());
        })
    };
    (*ws_mut_ref).borrow_mut().reschedule_timeout = Some(new_timeout);
}

/// Close websocket state: remove and drop websocket w/ its callbacks
fn cleanup_ws_state(ws_mut_ref: Rc<RefCell<WSState>>) {
    info!("cleaning up Websocket state");
    let mut ws_state = (*ws_mut_ref).borrow_mut();
    let ws = ws_state.ws.take();
    if let Some(ws) = ws {
        // remove event listeners
        ws.set_onclose(None);
        ws.set_onerror(None);
        ws.set_onopen(None);
        ws.set_onmessage(None);
        // close if it wasn't already
        ws.close().ok();
        drop(ws);
    }
    drop(ws_state.ws_callbacks.take());
}

/// Sends a message to the server via websocket
fn send_ws_message(
    app_reducer_handle: UseReducerHandle<AppState>,
    ws_mut_ref: Rc<RefCell<WSState>>,
    msg_type: CTSMsgInternal,
) -> bool {
    let (ws_is_none, ws_is_alive, ws_is_closed) = {
        let ws_state = (*ws_mut_ref).borrow();
        let ws_is_closed = if let Some(ws) = &ws_state.ws {
            match ws.ready_state() {
                WebSocket::CONNECTING => false,
                WebSocket::OPEN => false,
                WebSocket::CLOSED => true,
                WebSocket::CLOSING => true,
                _ => {
                    warn!("Unexpected ws ready state encountered");
                    true
                }
            }
        } else {
            true
        };
        (ws_state.ws.is_none(), ws_state.is_alive, ws_is_closed)
    };
    match msg_type {
        CTSMsgInternal::Test => {
            _send_ws_message(
                ws_mut_ref.clone(),
                CTSMsg::Test(String::from("Hello, server!")),
            );
            false
        }
        CTSMsgInternal::Ping => {
            let should_reconnect = if ws_is_none {
                info!(
                    "Trying to ping, but there is no websocket connection. Attempting to reconnect"
                );
                true
            } else if !ws_is_alive || ws_is_closed {
                info!("Trying to ping, but websocket is not alive or has closed. Dropping websocket connection and attempting to reconnect.");
                cleanup_ws_state(ws_mut_ref.clone());
                // ping continues retrying, so no need to reschedule a retry here
                true
            } else {
                false
            };

            // if is_alive is still false on next Ping, then the websocket is unresponsive
            (*ws_mut_ref).borrow_mut().is_alive = false;
            if should_reconnect {
                connect_to_ws(app_reducer_handle.clone(), ws_mut_ref.clone());
            } else {
                _send_ws_message(ws_mut_ref.clone(), CTSMsg::Ping);
            }
            false
        }
        CTSMsgInternal::Pong => {
            _send_ws_message(ws_mut_ref.clone(), CTSMsg::Pong);
            false
        }
        CTSMsgInternal::CreateGame => {
            if !app_reducer_handle.can_create_game() {
                return false;
            }

            // reset any errors on display name field
            app_reducer_handle.dispatch(AppReducerAction::SetDisplayNameInputError(None));

            let msg = CTSMsg::CreateGame {
                user_id: (*app_reducer_handle).user_id.clone(),
                display_name: (*app_reducer_handle).display_name_input.clone(),
            };
            _send_ws_message(ws_mut_ref.clone(), msg);
            false
        }
        CTSMsgInternal::JoinGameWithGameCode => {
            if !(*app_reducer_handle).can_join_game() {
                return false;
            }

            // reset any errors on display name field
            app_reducer_handle.dispatch(AppReducerAction::SetDisplayNameInputError(None));

            let msg = CTSMsg::JoinGameWithGameCode {
                game_code: (*app_reducer_handle)
                    .join_room_game_code_input
                    .clone()
                    .to_uppercase(),
                display_name: (*app_reducer_handle).display_name_input.clone(),
                user_id: (*app_reducer_handle).user_id.clone(),
            };

            _send_ws_message(ws_mut_ref.clone(), msg);
            false
        }
        CTSMsgInternal::LeaveGame => {
            if !(*app_reducer_handle).can_leave_game() {
                return false;
            }

            _send_ws_message(ws_mut_ref.clone(), CTSMsg::LeaveGame);
            false
        }
        CTSMsgInternal::MoveToTeam(team_option) => {
            _send_ws_message(ws_mut_ref.clone(), CTSMsg::MoveToTeam(team_option));
            false
        }
        CTSMsgInternal::RenameTeam(team_option) => {
            let team_name_input_clone = match &team_option {
                TeamOption::TeamA => (*app_reducer_handle).team_a_name_input.clone(),
                TeamOption::TeamB => (*app_reducer_handle).team_b_name_input.clone(),
            };

            let team_index = match &team_option {
                TeamOption::TeamA => 0,
                TeamOption::TeamB => 1,
            };

            // if team name input is empty on blur, replace with existing state and do not try to update on server
            if team_name_input_clone.is_empty() {
                let existing_team_name =
                    match &(*app_reducer_handle).game_state.as_ref().unwrap().stage {
                        PublicGameStage::Teams(teams_state) => {
                            teams_state[team_index].team_name.clone()
                        }
                        // not in teams stage, do nothing
                        _ => return false,
                    };
                app_reducer_handle.dispatch(match &team_option {
                    TeamOption::TeamA => AppReducerAction::SetTeamANameInput(existing_team_name),
                    TeamOption::TeamB => AppReducerAction::SetTeamBNameInput(existing_team_name),
                });
                return true;
            } else if validate_team_name(&team_name_input_clone).is_some() {
                return false;
            }

            _send_ws_message(
                ws_mut_ref.clone(),
                CTSMsg::RenameTeam {
                    team_name: team_name_input_clone,
                    team_option,
                },
            );

            false
        }
        CTSMsgInternal::StartGrandTichu => {
            if !(*app_reducer_handle).can_start_game() {
                warn!(
                    "State is not ready to start game. Ignoring request to send websocket message."
                );
                return false;
            }
            _send_ws_message(ws_mut_ref.clone(), CTSMsg::StartGrandTichu);
            false
        }
        CTSMsgInternal::CallGrandTichu(call_grand_tichu_request) => {
            if !(*app_reducer_handle).can_call_or_decline_grand_tichu() {
                return false;
            }

            _send_ws_message(
                ws_mut_ref.clone(),
                CTSMsg::CallGrandTichu(call_grand_tichu_request),
            );
            true
        }
        CTSMsgInternal::CallSmallTichu => {
            _send_ws_message(ws_mut_ref.clone(), CTSMsg::CallSmallTichu);
            false
        }
        CTSMsgInternal::SubmitTrade => {
            if !(*app_reducer_handle).can_submit_trade() {
                warn!("Invalid state to submit trade");
                return false;
            }

            // extract user_ids from Trade state
            let (teammate_user_id, opponent1_user_id, opponent2_user_id) =
                if let Some(game_state) = &(*app_reducer_handle).game_state {
                    if let PublicGameStage::Trade(trade_state) = &game_state.stage {
                        let current_team = trade_state
                            .teams
                            .iter()
                            .find(|team| team.user_ids.contains(&(*app_reducer_handle).user_id))
                            .expect("Error finding current user's in Team state");
                        let opposing_team = trade_state
                            .teams
                            .iter()
                            .find(|team| {
                                team.user_ids
                                    .iter()
                                    .all(|user_id| *user_id != (*app_reducer_handle).user_id)
                            })
                            .expect("Error finding opposing team in Team state");

                        (
                            current_team
                                .user_ids
                                .iter()
                                .find(|user_id| **user_id != *(*app_reducer_handle).user_id)
                                .expect("Error retrieving teammate's user_id from team state"),
                            opposing_team.user_ids[0].clone(),
                            opposing_team.user_ids[1].clone(),
                        )
                    } else {
                        warn!("Can't submit trade since current game stage is not Trade");
                        return false;
                    }
                } else {
                    warn!("Can't submit trade since current game state is None");
                    return false;
                };

            // clear selected card / trade state
            app_reducer_handle.dispatch(AppReducerAction::RemoveSelectedPrePlayCard);
            app_reducer_handle
                .dispatch(AppReducerAction::RemoveTrade(OtherPlayerOption::Opponent1));
            app_reducer_handle.dispatch(AppReducerAction::RemoveTrade(OtherPlayerOption::Teammate));
            app_reducer_handle
                .dispatch(AppReducerAction::RemoveTrade(OtherPlayerOption::Opponent2));

            // create SubmitTrade body data
            let submit_trade = [
                CardTrade {
                    to_user_id: opponent1_user_id,
                    card: (*app_reducer_handle)
                        .trade_to_opponent1
                        .as_ref()
                        .unwrap()
                        .clone(),
                    from_user_id: (*app_reducer_handle).user_id.clone(),
                },
                CardTrade {
                    to_user_id: teammate_user_id.to_string(),
                    card: (*app_reducer_handle)
                        .trade_to_teammate
                        .as_ref()
                        .unwrap()
                        .clone(),
                    from_user_id: (*app_reducer_handle).user_id.clone(),
                },
                CardTrade {
                    to_user_id: opponent2_user_id,
                    card: (*app_reducer_handle)
                        .trade_to_opponent2
                        .as_ref()
                        .unwrap()
                        .clone(),
                    from_user_id: (*app_reducer_handle).user_id.clone(),
                },
            ];

            _send_ws_message(ws_mut_ref.clone(), CTSMsg::SubmitTrade(submit_trade));
            false
        }
        CTSMsgInternal::PlayCards => {
            if !(*app_reducer_handle).can_play_cards() {
                return false;
            }

            let cards = (*app_reducer_handle).selected_play_cards.clone();
            // CardValue::noop() is equivalent to None
            let wished_for_card_value =
                if (*app_reducer_handle).wished_for_card_value == CardValue::noop() {
                    None
                } else {
                    Some((*app_reducer_handle).wished_for_card_value.clone())
                };
            let user_id_to_give_dragon_to = (*app_reducer_handle).user_id_to_give_dragon_to.clone();

            // reset state
            app_reducer_handle.dispatch(AppReducerAction::ResetAfterPlayCards);

            _send_ws_message(
                ws_mut_ref.clone(),
                CTSMsg::PlayCards {
                    cards,
                    wished_for_card_value,
                    user_id_to_give_dragon_to,
                },
            );
            false
        }
        CTSMsgInternal::Pass => {
            if !(*app_reducer_handle).get_can_pass() {
                // todo!() -- rethink this flow. Is it necessary?
                // user must select an opponent to give the dragon to
                if (*app_reducer_handle).get_user_must_select_user_id_to_give_dragon_to()
                    && !(*app_reducer_handle).get_user_has_selected_user_id_to_give_dragon_to()
                {
                    app_reducer_handle
                        .dispatch(AppReducerAction::SetShowUserIdToGiveDragonToForm(true));
                }
                return true;
            }
            _send_ws_message(ws_mut_ref.clone(), CTSMsg::Pass);
            app_reducer_handle.dispatch(AppReducerAction::SetUserIdToGiveDragonTo(None));
            app_reducer_handle.dispatch(AppReducerAction::SetShowUserIdToGiveDragonToForm(false));
            true
        }
        CTSMsgInternal::__AdminSkipToPlay => {
            _send_ws_message(ws_mut_ref.clone(), CTSMsg::__AdminSkipToPlay);
            false
        }
    }
}

/// Helper function to actually send the websocket message
fn _send_ws_message(ws_mut_ref: Rc<RefCell<WSState>>, msg: CTSMsg) {
    let ws_state = (*ws_mut_ref).borrow();
    match ws_state.ws {
        None => {
            warn!("Can't send message. Websocket is None in state");
        }
        Some(ref ws) => {
            {
                let msg = bincode::serialize(&msg).expect("Could not serialize message");
                ws.send_with_u8_array(&msg)
                    .expect("Error sending websocket data as u8 array over websocket");
            }

            js_log_with_styling(
                "%cWS MSG SENT:",
                "background-color: #060A42; color: white; padding: 0 5px;",
                &JsValue::from_serde(&msg).unwrap(),
            );
        }
    }
}

pub fn use_setup_app_ws(
    app_reducer_handle: UseReducerHandle<AppState>,
) -> Callback<CTSMsgInternal> {
    let ws_mut_ref = use_mut_ref(|| WSState::default());

    // connect to ws and begin pinging server once app has mounted
    {
        let ws_mut_ref = ws_mut_ref.clone();
        let app_reducer_handle = app_reducer_handle.clone();
        use_effect_with_deps(
            move |_| {
                connect_to_ws(app_reducer_handle.clone(), ws_mut_ref.clone());

                // cleanup function ?
                || {}
            },
            (),
        );
    }

    // create a callback for child components to be able to use to send websocket messages directly
    let app_reducer_handle = app_reducer_handle.clone();
    let ws_mut_ref = ws_mut_ref.clone();
    Callback::from(move |msg: CTSMsgInternal| {
        send_ws_message(app_reducer_handle.clone(), ws_mut_ref.clone(), msg);
    })
}

/// Handles when a websocket message is received from the server
/// Returns whether the component should re-render or not
fn handle_ws_message_received(
    app_reducer_handle: UseReducerHandle<AppState>,
    ws_mut_ref: Rc<RefCell<WSState>>,
    data: Result<Vec<u8>, Error>,
) -> bool {
    let mut should_rerender = true;
    if data.is_err() {
        error!("Data received from websocket was an error {:#?}", &data);
        return false;
    }
    let data: Option<STCMsg> = bincode::deserialize(&data.unwrap()).ok();

    js_log_with_styling(
        "%cWS MSG RECEIVED:",
        "background-color: #3A408C; color: white; padding: 0 5px;",
        &JsValue::from_serde(&data).unwrap(),
    );

    // any valid message received from the server indicates that the websocket is still alive
    (*ws_mut_ref).borrow_mut().is_alive = true;

    match data {
        None => {
            warn!("Deserialized data is None. This probably indicates there was an error deserializing the websocket message binary");
        }
        Some(data) => match data {
            STCMsg::Ping => {
                send_ws_message(app_reducer_handle, ws_mut_ref, CTSMsgInternal::Pong);
            }
            STCMsg::UserIdAssigned(s) => {
                app_reducer_handle.dispatch(AppReducerAction::SetUserId(s));
            }
            STCMsg::GameState(new_game_state) => {
                let new_game_state = *new_game_state;

                // if team names are empty, update team name inputs to reflect state
                if (*app_reducer_handle).team_a_name_input.is_empty()
                    || (*app_reducer_handle).team_b_name_input.is_empty()
                {
                    if let Some(new_game_state) = &new_game_state {
                        if let PublicGameStage::Teams(teams_state) = &new_game_state.stage {
                            app_reducer_handle.dispatch(AppReducerAction::SetTeamANameInput(
                                (*teams_state[0].team_name).to_string(),
                            ));
                            app_reducer_handle.dispatch(AppReducerAction::SetTeamBNameInput(
                                (*teams_state[1].team_name).to_string(),
                            ));
                        }
                    }
                }

                // move into block and back out for mutability (TODO: clean up later)
                let new_game_state = if let Some(mut new_game_state) = new_game_state {
                    // save display name input to state/localStorage
                    app_reducer_handle.dispatch(AppReducerAction::SetDisplayName(
                        (*new_game_state.current_user.display_name).to_string(),
                    ));

                    // sort current user's hand
                    sort_cards_for_hand(&mut new_game_state.current_user.hand);

                    Box::new(Some(new_game_state))
                } else {
                    Box::new(new_game_state)
                };

                app_reducer_handle.dispatch(AppReducerAction::SetGameState(*new_game_state));
                should_rerender = true;
            }
            STCMsg::UnexpectedMessageReceived(s) => {
                warn!(
                    "Server received unexpected message from client. Message sent from client: {}",
                    s
                );
            }
            STCMsg::Pong => { /* is_alive is set to true if ANY message is received */ }
            STCMsg::TeamARenamed(new_team_a_name) => {
                app_reducer_handle.dispatch(AppReducerAction::SetTeamANameInput(new_team_a_name));
            }
            STCMsg::TeamBRenamed(new_team_b_name) => {
                app_reducer_handle.dispatch(AppReducerAction::SetTeamBNameInput(new_team_b_name));
            }
            STCMsg::Test(_) => {}
            STCMsg::GameCreated { .. } => {}
            STCMsg::UserJoined(_) => {}
            STCMsg::UserDisconnected(_) => {}
            STCMsg::UserReconnected(_) => {}
            STCMsg::UserLeft(_) => {}
            STCMsg::OwnerReassigned(_) => {}
            STCMsg::UserMovedToTeamA(_) => {}
            STCMsg::UserMovedToTeamB(_) => {}
            STCMsg::GameStageChanged(_) => {}
            STCMsg::GrandTichuCalled(_, _) => {}
            STCMsg::SmallTichuCalled(_) => {}
            STCMsg::TradeSubmitted(_) => {}
            STCMsg::CardsPlayed => {}
            STCMsg::FirstCardsDealt => {}
            STCMsg::LastCardsDealt => {}
            STCMsg::PlayerReceivedDragon => {}
            STCMsg::GameEnded => {}
            STCMsg::GameEndedFinal => {}
            STCMsg::UserPassed(_) => {}
        },
    }

    should_rerender
}
