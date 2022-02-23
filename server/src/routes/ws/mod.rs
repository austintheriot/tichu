mod cleanup_state_after_disconnect;
mod handle_message_received;
pub mod send_ws_message;
use crate::{ConnectionData, Connections, GameCodes, Games};
pub use cleanup_state_after_disconnect::cleanup_state_after_disconnect;
use common::{STCMsg, NO_USER_ID};
use futures::{SinkExt, StreamExt};
pub use handle_message_received::handle_message_received;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;
use warp::ws::{Message, WebSocket};

pub async fn handle_ws_upgrade(
    ws: WebSocket,
    user_id: String,
    connections: Connections,
    games: Games,
    game_codes: GameCodes,
) {
    eprintln!("User {user_id}: Connected");

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
            if Message::is_close(&message) {
                let result = user_ws_tx.close().await;
                if let Err(e) = result {
                    eprintln!("Error closing websocket {:#?}", e);
                }
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
                let result = user_ws_tx.send(message).await;
                if let Err(e) = result {
                    eprintln!("Websocket send error: {}", e);
                    let result = user_ws_tx.close().await;
                    if let Err(e) = result {
                        eprintln!("Error closing websocket {:#?}", e);
                    }
                }
            }
        }
    });

    eprintln!("User {user_id}: Waiting for RWLock to Connections");
    let mut write_connections = connections.write().await;
    eprintln!("User {user_id}: Connections RWLock received");

    let is_a_returning_user = write_connections.contains_key(&user_id);
    let mut new_user_id_assigned = false;
    let mut game_id = None;

    // only reassign user_ids if the user hasn't claimed one yet
    // they can use whichever one they provide as long as it is unique
    let user_id = if user_id == NO_USER_ID {
        new_user_id_assigned = true;
        let new_user_id = Uuid::new_v4().to_string();
        eprintln!("User {user_id}: New user_id: {new_user_id}");
        new_user_id
    } else {
        user_id
    };

    // get associated game_id from returning users
    if is_a_returning_user {
        eprintln!("User {user_id} is a returning user");

        if let Some(connection_data) = write_connections.get(&user_id) {
            game_id = connection_data.game_id.clone();
        } else {
            eprintln!("User {user_id}: error: Returning user should be in write_connections");
            return;
        }
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
        send_ws_message::to_user(
            &user_id,
            STCMsg::UserIdAssigned(user_id.clone()),
            &connections,
        )
        .await;
    }

    // this user is already saved in Connections
    if is_a_returning_user {
        if let Some(game_id) = game_id {
            // notify other participants (if any) that the user reconnected
            send_ws_message::to_group(
                &game_id,
                STCMsg::UserReconnected(user_id.to_string()),
                &connections,
                &games,
                &game_codes,
            )
            .await;

            // if user is associated with a game_id, send the new participant a state update
            let read_games = games.read().await;
            let game_state = read_games
                .get(&game_id)
                .and_then(|game_state| game_state.to_public_game_state(&user_id).ok());
            drop(read_games);

            send_ws_message::to_user(
                &user_id,
                STCMsg::GameState(Box::new(game_state)),
                &connections,
            )
            .await;
        }
    } else {
        // send them a None state update to clear any lingering local state
        send_ws_message::to_user(&user_id, STCMsg::GameState(Box::new(None)), &connections).await;
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
        handle_message_received(
            user_id.clone(),
            msg,
            Arc::clone(&connections),
            Arc::clone(&games),
            Arc::clone(&game_codes),
        )
        .await;
    }

    // handle if user disconnects
    cleanup_state_after_disconnect(&user_id, &connections, &games, &game_codes).await;
}
