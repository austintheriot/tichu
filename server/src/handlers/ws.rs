use crate::{Games, Websockets};
use bincode;
use common::CTSMsg;
use futures::{SinkExt, StreamExt, TryFutureExt};
use tokio::sync::{mpsc};
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;
use warp::ws::{Message, WebSocket};

pub async fn handle_ws_upgrade(
    ws: WebSocket,
    user_id: String,
    websockets: Websockets,
    games: Games,
) {
    eprintln!("User connected: {}", user_id);

    let (mut user_ws_tx, mut user_ws_rx) = ws.split();
    let (tx, rx) = mpsc::unbounded_channel();
    let mut rx = UnboundedReceiverStream::new(rx);

    tokio::task::spawn(async move {
        while let Some(message) = rx.next().await {
            user_ws_tx
                .send(message)
                .unwrap_or_else(|e| {
                    eprintln!("websocket send error: {}", e);
                })
                .await;
        }
    });

    let user_id = if user_id == "no_id" {
        Uuid::new_v4().to_string()

        // NOTE: make sure to tell client that a new username has been assigned!
        // ****************************************************************
        // ****************************************************************
        // ****************************************************************
    } else {
        user_id
    };

    // Save the sender in our list of connected users.
    websockets.write().await.insert(user_id.clone(), tx);

    // Listen for incoming messages
    while let Some(result) = user_ws_rx.next().await {
        let msg = match result {
            Ok(msg) => {
                msg
            }
            Err(_) => {
                break;
            }
        };
        handle_message_received(user_id.clone(), msg, &websockets, &games).await;
    }

    // handle if user disconnects
    user_disconnected(user_id, &websockets).await;
}

pub async fn handle_message_received(
    user_id: String,
    msg: Message,
    users: &Websockets,
    games: &Games,
) {
    if !msg.is_binary() {
        eprint!("Messaged received is not binary: {:?}", &msg);
        return;
    }

    let msg: CTSMsg = bincode::deserialize(&msg.as_bytes()).expect("Could not serialize message");

    match msg {
        CTSMsg::Test(_) => {
            send_message(
                user_id,
                CTSMsg::Test("Hello client!".into()),
                &users,
                &games,
            )
            .await;
        }
        CTSMsg::Ping => {
            send_message(user_id, CTSMsg::Pong, &users, &games).await;
        }
        _ => eprint!("Unexpected message received\n"),
    }
}

pub async fn send_message(user_id: String, msg: CTSMsg, users: &Websockets, games: &Games) {
    let msg = bincode::serialize(&msg).expect("Could not serialize message");
    let msg = Message::binary(msg);
    let users = users.read().await;
    let tx = users
        .get(&user_id)
        .expect("Couldn't retrieve userId from users HashMap");
    if let Err(_disconnected) = tx.send(msg.clone()) {
        eprint!("User is disconnected. Couldn't send message {:?}", &msg);
    } else {
        eprint!("Response successfully sent");
    }
}

pub async fn update_game_state(user_id: String, msg: Message, users: &Websockets, games: &Games) {}

pub async fn user_disconnected(user_id: String, users: &Websockets) {
    eprintln!("User disconnected: {}", user_id);

    // Stream closed up, so remove from the user list
    users.write().await.remove(&user_id);

    // Clean up game state?
}
