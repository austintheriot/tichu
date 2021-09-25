mod cleanup_state_after_disconnect;
mod handle_message_received;
mod send_game_state_to_all_participants;
mod send_ws_message_to_all_participants;
mod send_ws_message_to_user;

pub use cleanup_state_after_disconnect::cleanup_state_after_disconnect;
pub use handle_message_received::handle_message_received;
pub use send_game_state_to_all_participants::send_game_state_to_all_participants;
pub use send_ws_message_to_all_participants::send_ws_message_to_all_participants;
pub use send_ws_message_to_user::send_ws_message_to_user;

use crate::{
    errors::{GAME_ID_NOT_IN_MAP, USER_ID_NOT_IN_MAP},
    ConnectionData, Connections, GameCodes, Games,
};
use common::{STCMsg, NO_USER_ID};
use futures::{SinkExt, StreamExt, TryFutureExt};
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;
use warp::ws::{Message, WebSocket};

pub static CLOSE_WEBSOCKET: &str = "INTERNAL_MESSAGE: CLOSE WEBSOCKET";

pub async fn handle_ws_upgrade(
    ws: WebSocket,
    user_id: String,
    connections: Connections,
    games: Games,
    game_codes: GameCodes,
) {
    eprintln!("User connected! Original user_id = {}", user_id);

    let (mut user_ws_tx, mut user_ws_rx) = ws.split();
    // use a channel to send messages to our websocket sink (sender)
    let (tx, rx) = mpsc::unbounded_channel();
    let mut rx = UnboundedReceiverStream::new(rx);

    let user_id_clone = user_id.clone();
    let users_clone = Arc::clone(&connections);
    let games_clone = Arc::clone(&games);
    let game_codes_clone = Arc::clone(&game_codes);

    // send any incoming stream messages to client
    tokio::task::spawn(async move {
        while let Some(message) = rx.next().await {
            // user didn't respond to ping: close connection
            if message == Message::text(CLOSE_WEBSOCKET) {
                user_ws_tx
                    .close()
                    .await
                    .expect("Could not close websocket for idle connection");
                cleanup_state_after_disconnect(
                    &user_id_clone,
                    &users_clone,
                    &games_clone,
                    &game_codes_clone,
                )
                .await;
            }
            // take WS from queue and send to client
            else {
                user_ws_tx
                    .send(message)
                    .unwrap_or_else(|e| {
                        eprintln!("Websocket send error: {}", e);
                    })
                    .await;
            }
        }
    });

    let mut write_connections = connections.write().await;
    let is_a_returning_user = write_connections.contains_key(&user_id);
    let mut new_user_id_assigned = false;
    let mut game_id = None;

    // user either has no user_id, or does not have user_id that is currently saved in memory
    let user_id = if user_id == NO_USER_ID || !is_a_returning_user {
        new_user_id_assigned = true;
        Uuid::new_v4().to_string()
    } else {
        user_id
    };

    eprintln!("Final user_id = {}", user_id);

    // get associated game_id from returning users
    if is_a_returning_user {
        eprintln!("User {} reconnected", user_id);
        let existing_user = write_connections.get(&user_id).expect(USER_ID_NOT_IN_MAP);
        game_id = existing_user.game_id.clone();
    }

    let ws = ConnectionData {
        user_id: user_id.clone(),
        game_id: game_id.clone(),
        tx,
        is_alive: Arc::new(RwLock::new(true)),
        connected: true,
    };

    // associate user_id to websocket
    write_connections.insert(user_id.clone(), ws);
    drop(write_connections);

    // must be saved under new user_id before sending message
    if new_user_id_assigned {
        send_ws_message_to_user(
            &user_id,
            STCMsg::UserIdAssigned(user_id.clone()),
            &connections,
        )
        .await;
    }

    // this user is already saved in Connections
    if is_a_returning_user {
        match game_id {
            Some(game_id) => {
                // notify other participants (if any) that the user reconnected
                send_ws_message_to_all_participants(
                    &game_id,
                    STCMsg::UserReconnected(user_id.to_string()),
                    &connections,
                    &games,
                    &game_codes,
                )
                .await;

                // if user is associated with a game_id, send the new participant a state update
                let read_games = games.read().await;
                let game_state = read_games.get(&game_id).expect(GAME_ID_NOT_IN_MAP).clone();
                drop(read_games);
                send_ws_message_to_user(
                    &user_id,
                    STCMsg::GameState(game_state.to_public_game_state(&user_id)),
                    &connections,
                )
                .await;
            }
            _ => {}
        }
    } else {
        // send them a None state update to clear any lingering local state
        send_ws_message_to_user(&user_id, STCMsg::GameState(None), &connections).await;
    }

    // Listen for incoming messages
    while let Some(result) = user_ws_rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("Received error message: {:#?}", e);
                break;
            }
        };
        handle_message_received(user_id.clone(), msg, &connections, &games, &game_codes).await;
    }

    // handle if user disconnects
    cleanup_state_after_disconnect(&user_id, &connections, &games, &game_codes).await;
}
