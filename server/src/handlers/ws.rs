use crate::{AppWebSocket, Games, Websockets};
use bincode;
use common::{CTSMsg, STCMsg};
use futures::{SinkExt, StreamExt, TryFutureExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{mpsc, RwLock};
use tokio::{task, time};
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;
use warp::ws::{Message, WebSocket};

pub static CLOSE_WEBSOCKET: &str = "INTERNAL_MESSAGE: CLOSE WEBSOCKET";

pub async fn handle_ws_upgrade(ws: WebSocket, user_id: String, users: Websockets, games: Games) {
    eprintln!("User connected! user_id = {}", user_id);

    let (mut user_ws_tx, mut user_ws_rx) = ws.split();
    let (tx, rx) = mpsc::unbounded_channel();
    let mut rx = UnboundedReceiverStream::new(rx);

    let user_id_clone = user_id.clone();
    let users_clone = users.clone();
    // send any messages that are waiting to be sent ?
    tokio::task::spawn(async move {
        while let Some(message) = rx.next().await {
            // user didn't respond to ping: close connection
            if message == Message::text(CLOSE_WEBSOCKET) {
                user_ws_tx.close();
                user_disconnected(&user_id_clone, &users_clone).await;
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

    let mut new_user_id_assigned = false;
    let user_id = if user_id == "NO_USER_ID" {
        new_user_id_assigned = true;
        Uuid::new_v4().to_string()
    } else {
        user_id
    };

    let ws = AppWebSocket {
        is_alive: Arc::new(RwLock::new(true)),
        tx,
    };

    // Save the sender in our list of connected users.
    users.write().await.insert(user_id.clone(), ws);

    // must be saved under new user_id before sending message
    if (new_user_id_assigned) {
        send_message(
            user_id.clone(),
            STCMsg::UserIdAssigned(user_id.clone()),
            &users,
            &games,
        )
        .await;
    }

    // Listen for incoming messages
    while let Some(result) = user_ws_rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(_) => {
                break;
            }
        };
        handle_message_received(user_id.clone(), msg, &users, &games).await;
    }

    // handle if user disconnects
    user_disconnected(&user_id, &users).await;
}

pub async fn handle_message_received(
    user_id: String,
    msg: Message,
    users: &Websockets,
    games: &Games,
) {
    if !msg.is_binary() {
        eprint!("Unexpected text websocket message received: {:?}", &msg);
        return;
    }

    let msg: CTSMsg = bincode::deserialize(&msg.as_bytes()).expect("Could not serialize message");

    match msg {
        CTSMsg::Test(_) => {
            send_message(
                user_id,
                STCMsg::Test("Hello client!".into()),
                &users,
                &games,
            )
            .await;
        }
        CTSMsg::Ping => {
            send_message(user_id, STCMsg::Pong, &users, &games).await;
        }
        CTSMsg::Pong => {
            let mut users = users.write().await;
            let mut ws = users
                .get(&user_id)
                .expect("User id not found in websocket hashmap");
            let mut is_alive = ws.is_alive.write().await;
            *is_alive = true;
        }
        _ => eprint!("Unexpected message received: {:?}\n", msg),
    }
}

pub async fn send_message(user_id: String, msg: STCMsg, users: &Websockets, games: &Games) {
    let msg = bincode::serialize(&msg).expect("Could not serialize message");
    let msg = Message::binary(msg);
    let users = users.read().await;
    let ws = users
        .get(&user_id)
        .expect("Couldn't retrieve userId from users HashMap");
    if let Err(_disconnected) = ws.tx.send(msg.clone()) {
        eprint!("User is disconnected. Couldn't send message {:?}\n", &msg);
    } else {
        eprint!("Message successfully sent\n");
    }
}

pub async fn user_disconnected(user_id: &String, users: &Websockets) {
    eprintln!("User disconnected: {}", user_id);

    // Stream closed up, so remove from the user list
    users.write().await.remove(user_id);

    // Clean up game state?
}
