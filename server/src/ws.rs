use futures::{SinkExt, StreamExt, TryFutureExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;
use warp::ws::{Message, WebSocket};

use crate::{Games, Websockets};

pub async fn handle_ws_upgrade(ws: WebSocket, websockets: Websockets, games: Games) {
    // Use a counter to assign a new unique ID for this user.
    let user_uuid = Uuid::new_v4();

    eprintln!("User connected: {}", user_uuid);

    // Split the socket into a sender and receive of messages.
    let (mut user_ws_tx, mut user_ws_rx) = ws.split();

    // Use an unbounded channel to handle buffering and flushing of messages
    // to the websocket...
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

    // Save the sender in our list of connected users.
    websockets.write().await.insert(user_uuid, tx);

    // Return a `Future` that is basically a state machine managing
    // this specific user's connection.

    // Every time the user sends a message, broadcast it to
    // all other users...
    while let Some(result) = user_ws_rx.next().await {
        let msg = match result {
            Ok(msg) => {
                eprintln!("Message received from {}: {:?}", &user_uuid, &msg);
                msg
            }
            Err(e) => {
                eprintln!("websocket error(uid={}): {}", user_uuid, e);
                break;
            }
        };
        handle_message_received(user_uuid, msg, &websockets, &games).await;
        // send_message(user_uuid, msg, &websockets, &games).await;
    }

    // user_ws_rx stream will keep processing as long as the user stays
    // connected. Once they disconnect, then...
    user_disconnected(user_uuid, &websockets).await;
}

pub async fn handle_message_received(
    user_id: Uuid,
    msg: Message,
    users: &Websockets,
    games: &Games,
) {
    let msg = if let Ok(s) = msg.to_str() {
        s
    } else {
        return;
    };

    if let "create_game" = msg {
        eprint!("Create game message received!");
    }
}

pub async fn update_game_state(user_id: Uuid, msg: Message, users: &Websockets, games: &Games) {}

pub async fn send_message(user_id: Uuid, msg: Message, users: &Websockets, games: &Games) {
    // Skip any non-Text messages...
    let msg = if let Ok(s) = msg.to_str() {
        s
    } else {
        return;
    };

    eprint!("Games: {:#?}", &games.read().await);
    let new_msg = format!("<User#{}>: {}", user_id, msg);

    // New message from this user, send it to everyone else (except same uid)...
    for (&uid, tx) in users.read().await.iter() {
        if user_id != uid {
            if let Err(_disconnected) = tx.send(Message::text(new_msg.clone())) {
                // The tx is disconnected, our `user_disconnected` code
                // should be happening in another task, nothing more to
                // do here.
            }
        }
    }
}

pub async fn user_disconnected(user_id: Uuid, users: &Websockets) {
    eprintln!("User disconnected: {}", user_id);

    // Stream closed up, so remove from the user list
    users.write().await.remove(&user_id);

    // Clean up game state?
}
