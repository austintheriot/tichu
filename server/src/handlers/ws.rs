use crate::{
    errors::{GAME_ID_NOT_IN_MAP, USER_ID_NOT_IN_MAP},
    ConnectionData, Connections, Games,
};
use bincode;
use common::{CTSMsg, CreateGame, GameState, STCMsg};
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
) {
    eprintln!("User connected! Original user_id = {}\n", user_id);

    let (mut user_ws_tx, mut user_ws_rx) = ws.split();
    // use a channel to send messages to our websocket sink (sender)
    let (tx, rx) = mpsc::unbounded_channel();
    let mut rx = UnboundedReceiverStream::new(rx);

    let user_id_clone = user_id.clone();
    let users_clone = Arc::clone(&connections);
    let games_clone = Arc::clone(&games);

    // send any incoming stream messages to client
    tokio::task::spawn(async move {
        while let Some(message) = rx.next().await {
            // user didn't respond to ping: close connection
            if message == Message::text(CLOSE_WEBSOCKET) {
                user_ws_tx
                    .close()
                    .await
                    .expect("Could not close websocket for idle connection");
                user_disconnected(&user_id_clone, &users_clone, &games_clone).await;
            }
            // take WS from queue and send to client
            else {
                user_ws_tx
                    .send(message)
                    .unwrap_or_else(|e| {
                        eprintln!("Websocket send error: {}\n", e);
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

    eprintln!("New user_id = {}\n", user_id);

    let ws = ConnectionData {
        user_id: user_id.clone(),
        game_id: None,
        is_alive: Arc::new(RwLock::new(true)),
        connected: true,
        tx,
    };

    // Save the sender in our list of connected users.
    let mut write_connections = connections.write().await;
    if write_connections.contains_key(&user_id) {
        eprint!("Warning: user_id {} already exists in HashMap. This likely indicates that an error occurred, and the user was not erased from memory properly", user_id);
    }
    write_connections.insert(user_id.clone(), ws);
    drop(write_connections);

    // must be saved under new user_id before sending message
    if new_user_id_assigned {
        send_message(
            &user_id,
            STCMsg::UserIdAssigned(user_id.clone()),
            &connections,
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
        handle_message_received(user_id.clone(), msg, &connections, &games).await;
    }

    // handle if user disconnects
    user_disconnected(&user_id, &connections, &games).await;
}

pub async fn handle_message_received(
    user_id: String,
    msg: Message,
    connections: &Connections,
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
                &user_id,
                STCMsg::Test("Hello client!".into()),
                &connections,
                &games,
            )
            .await;
        }
        CTSMsg::Ping => {
            send_message(&user_id, STCMsg::Pong, &connections, &games).await;
        }
        CTSMsg::Pong => {
            let connections = connections.read().await;
            let ws = connections.get(&user_id).expect(USER_ID_NOT_IN_MAP);
            let mut is_alive = ws.is_alive.write().await;
            *is_alive = true;
        }
        CTSMsg::CreateGame(create_game) => {
            let CreateGame {
                user_id,
                display_name,
            } = create_game;

            // verify that user_id is not already associated with a game before creating a new one
            let mut write_connections = connections.write().await;
            let connection = write_connections
                .get_mut(&user_id)
                .expect(USER_ID_NOT_IN_MAP);

            // user already associated with a game, no action needed
            if let Some(game_id) = &connection.game_id {
                eprint!(
                    "Can't create game for user: user is already associated with a game: {}\n",
                    game_id
                );
                return;
            }

            // user is NOT associated with a game: create game for user
            let game_state = GameState::new(user_id.clone(), display_name);
            let mut write_games = games.write().await;
            write_games.insert(game_state.game_id.clone(), game_state.clone());
            let _ = connection.game_id.insert(game_state.game_id.clone());

            // these must be dropped, or else deadlock occurs, because send_message
            // waits for read access, which is never given while these variables have
            // write access--which only get dropped once message is sent, and so on
            drop(write_games);
            drop(write_connections);

            // send updated new game state to owner
            eprint!("New game successfully created! {:#?}\n", &game_state);
            send_message(&user_id, STCMsg::GameCreated, &connections, &games).await;
            send_message(
                &user_id,
                STCMsg::GameState(game_state.clone()),
                &connections,
                &games,
            )
            .await;
        }
        _ => eprint!("Unexpected message received: {:?}\n", msg),
    }
}

pub async fn send_message(user_id: &String, msg: STCMsg, connections: &Connections, _: &Games) {
    let msg = bincode::serialize(&msg).expect("Could not serialize message");
    let msg = Message::binary(msg);
    let read_connections = connections.read().await;
    let ws = read_connections.get(user_id).expect(USER_ID_NOT_IN_MAP);
    if let Err(_disconnected) = ws.tx.send(msg.clone()) {
        eprint!("User is disconnected. Couldn't send message {:?}\n", &msg);
    } else {
        eprint!("Message successfully sent\n");
    }
}

pub async fn user_disconnected(user_id: &String, connections: &Connections, games: &Games) {
    eprintln!("User disconnected: {}\n", user_id);

    // extract game_id
    let mut write_connections = connections.write().await;
    let mut write_games = games.write().await;
    let game_id_clone = &write_connections
        .get(user_id)
        .expect(USER_ID_NOT_IN_MAP)
        .game_id
        .clone();

    match game_id_clone {
        Some(game_id) => {
            // get all user_ids of participants in the game
            let mut any_user_is_still_in_game = false;
            let participants = &write_games
                .get(game_id)
                .expect(GAME_ID_NOT_IN_MAP)
                .participants;

            // check if any are still connected
            for participant in participants.iter() {
                if &participant.user_id != user_id {
                    let participant_connection = write_connections
                        .get(&participant.user_id)
                        .expect(USER_ID_NOT_IN_MAP);
                    if participant_connection.connected {
                        any_user_is_still_in_game = true;
                    }
                }
            }

            // only mark this user as disconnected
            if any_user_is_still_in_game {
                eprint!("Marking {} as not connected\n", user_id);
                write_connections
                    .get_mut(user_id)
                    .expect(USER_ID_NOT_IN_MAP)
                    .connected = false;
            }
            // no users left in game: remove all users from Connections and delete game from Games
            else {
                eprint!(
                    "Removing all users and game from state for game {}\n",
                    game_id
                );
                for participant in participants.iter() {
                    write_connections.remove(&participant.user_id);
                }
                write_games.remove(game_id);
            }
        }
        None => {
            // user was not associated with a game (shouldn't happen), remove from Connections
            let mut write_connections = connections.write().await;
            write_connections.remove(user_id);
        }
    }
}
