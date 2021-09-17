use crate::{Games, Websockets};
use bincode;
use common::STCMsg;
use futures::{SinkExt, StreamExt, TryFutureExt};
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;
use warp::ws::{Message, WebSocket};

pub async fn handle_ws_upgrade(ws: WebSocket, user_id: String, users: Websockets, games: Games) {
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

    let mut new_user_id_assigned = false;
    let user_id = if user_id == "no_id" {
        new_user_id_assigned = true;
        Uuid::new_v4().to_string()
    } else {
        user_id
    };

    // Save the sender in our list of connected users.
    users.write().await.insert(user_id.clone(), tx);

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
    user_disconnected(user_id, &users).await;
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

    let msg: STCMsg = bincode::deserialize(&msg.as_bytes()).expect("Could not serialize message");

    match msg {
        STCMsg::Test(_) => {
            send_message(
                user_id,
                STCMsg::Test("Hello client!".into()),
                &users,
                &games,
            )
            .await;
        }
        STCMsg::Ping => {
            send_message(user_id, STCMsg::Pong, &users, &games).await;
        }
        _ => eprint!("Unexpected message received\n"),
    }
}

pub async fn send_message(user_id: String, msg: STCMsg, users: &Websockets, games: &Games) {
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
