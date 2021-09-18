use crate::{Games, UserData, Users};
use bincode;
use common::{CTSMsg, CreateGame, GameState, LobbyStage, STCMsg, User, UserRole};
use futures::{SinkExt, StreamExt, TryFutureExt};
use std::{
    mem::{replace, swap},
    sync::Arc,
};
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;
use warp::ws::{Message, WebSocket};

pub static CLOSE_WEBSOCKET: &str = "INTERNAL_MESSAGE: CLOSE WEBSOCKET";

pub async fn handle_ws_upgrade(ws: WebSocket, user_id: String, users: Users, games: Games) {
    eprintln!("User connected! user_id = {}", user_id);

    let (mut user_ws_tx, mut user_ws_rx) = ws.split();
    // use a channel to send messages to our websocket sink (sender)
    let (tx, rx) = mpsc::unbounded_channel();
    let mut rx = UnboundedReceiverStream::new(rx);

    let user_id_clone = user_id.clone();
    let users_clone = Arc::clone(&users);

    // send any incoming stream messages to client
    tokio::task::spawn(async move {
        while let Some(message) = rx.next().await {
            // user didn't respond to ping: close connection
            if message == Message::text(CLOSE_WEBSOCKET) {
                user_ws_tx
                    .close()
                    .await
                    .expect("Could not close websocket for idle user");
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

    let ws = UserData {
        is_alive: Arc::new(RwLock::new(true)),
        tx,
        game_id: None,
    };

    // Save the sender in our list of connected users.
    users.write().await.insert(user_id.clone(), ws);

    // must be saved under new user_id before sending message
    if (new_user_id_assigned) {
        send_message(
            &user_id,
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

pub async fn handle_message_received(user_id: String, msg: Message, users: &Users, games: &Games) {
    if !msg.is_binary() {
        eprint!("Unexpected text websocket message received: {:?}", &msg);
        return;
    }

    let msg: CTSMsg = bincode::deserialize(&msg.as_bytes()).expect("Could not serialize message");

    match msg {
        CTSMsg::Test(_) => {
            send_message(
                &user_id,
                STCMsg::Test("Hello client!".into()),
                &users,
                &games,
            )
            .await;
        }
        CTSMsg::Ping => {
            send_message(&user_id, STCMsg::Pong, &users, &games).await;
        }
        CTSMsg::Pong => {
            let mut users = users.write().await;
            let mut ws = users
                .get(&user_id)
                .expect("User id not found in websocket hashmap");
            let mut is_alive = ws.is_alive.write().await;
            *is_alive = true;
        }
        CTSMsg::CreateGame(create_game) => {
            let CreateGame {
                user_id,
                display_name,
            } = create_game;

            // verify that user_id is not already associated with a game before creating a new one
            let mut users_as_mut = users.write().await;
            let user = users_as_mut
                .get_mut(&user_id)
                .expect("User was not found in Users hashmap");

            // user already associated with a game, no action needed
            if let Some(game_id) = &user.game_id {
                eprint!(
                    "Can't create game for user: user is already associated with a game: {}",
                    game_id
                );
                return;
            }

            // user is NOT associated with a game: create game for user
            let game_state = GameState::new(user_id.clone(), display_name);
            let mut games_as_mut = games.write().await;
            games_as_mut.insert(game_state.id.clone(), game_state.clone());
            let _ = user.game_id.insert(game_state.id.clone());

            // these must be dropped, or else deadlock occurs, because send_message
            // waits for read access, which is never given while these variables have
            // write access--which only get dropped once message is sent, and so on
            drop(games_as_mut);
            drop(users_as_mut);

            // send updated new game state to owner
            eprint!("New game successfully created! {:#?}\n", &game_state);
            send_message(&user_id, STCMsg::GameCreated, &users, &games).await;
            send_message(
                &user_id,
                STCMsg::GameState(game_state.clone()),
                &users,
                &games,
            )
            .await;
        }
        _ => eprint!("Unexpected message received: {:?}\n", msg),
    }
}

pub async fn send_message(user_id: &String, msg: STCMsg, users: &Users, _: &Games) {
    let msg = bincode::serialize(&msg).expect("Could not serialize message");
    let msg = Message::binary(msg);
    let users = users.read().await;
    let ws = users
        .get(user_id)
        .expect("Couldn't retrieve userId from users HashMap");
    if let Err(_disconnected) = ws.tx.send(msg.clone()) {
        eprint!("User is disconnected. Couldn't send message {:?}\n", &msg);
    } else {
        eprint!("Message successfully sent\n");
    }
}

pub async fn user_disconnected(user_id: &String, users: &Users) {
    eprintln!("User disconnected: {}", user_id);

    // Stream closed up, so remove from the user list
    users.write().await.remove(user_id);

    // Clean up game state?
}
