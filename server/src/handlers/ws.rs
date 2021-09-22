use crate::{
    errors::{GAME_CODE_NOT_IN_MAP, GAME_ID_NOT_IN_MAP, USER_ID_NOT_IN_MAP},
    ConnectionData, Connections, GameCodes, Games,
};
use bincode;
use common::{CTSMsg, CreateGame, GameCreated, GameStage, GameState, JoinGameWithGameCode, STCMsg};
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
    eprintln!("User connected! Original user_id = {}\n", user_id);

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

    let mut user_reconnected = false;
    let mut game_id = None;

    let mut write_connections = connections.write().await;
    if write_connections.contains_key(&user_id) {
        user_reconnected = true;
        eprint!("User {} reconnected", user_id);
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

    // Associate user_id to game_id (if relevant) & websocket sender
    write_connections.insert(user_id.clone(), ws);
    drop(write_connections);

    // must be saved under new user_id before sending message
    if new_user_id_assigned {
        send_ws_message_to_user(
            &user_id,
            STCMsg::UserIdAssigned(user_id.clone()),
            &connections,
            &games,
            &game_codes,
        )
        .await;
    }

    if user_reconnected {
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
                    STCMsg::GameState(Some(game_state)),
                    &connections,
                    &games,
                    &game_codes,
                )
                .await;
            }
            _ => {}
        }
    }

    // Listen for incoming messages
    while let Some(result) = user_ws_rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(_) => {
                break;
            }
        };
        handle_message_received(user_id.clone(), msg, &connections, &games, &game_codes).await;
    }

    // handle if user disconnects
    cleanup_state_after_disconnect(&user_id, &connections, &games, &game_codes).await;
}

pub async fn handle_message_received(
    user_id: String,
    msg: Message,
    connections: &Connections,
    games: &Games,
    game_codes: &GameCodes,
) {
    if !msg.is_binary() {
        eprint!("Text websocket message received: {:?}\n", &msg);
        return;
    }

    let msg: CTSMsg = bincode::deserialize(&msg.as_bytes()).expect("Could not serialize message");

    match msg {
        CTSMsg::Test(_) => {
            send_ws_message_to_user(
                &user_id,
                STCMsg::Test("Hello client!".into()),
                &connections,
                &games,
                &game_codes,
            )
            .await;
        }
        CTSMsg::Ping => {
            send_ws_message_to_user(&user_id, STCMsg::Pong, &connections, &games, &game_codes)
                .await;
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
            let read_game_codes = game_codes.read().await;
            let game_state = GameState::new(user_id.clone(), display_name, &read_game_codes);
            drop(read_game_codes);

            // save game to state
            let mut write_games = games.write().await;
            write_games.insert(game_state.game_id.clone(), game_state.clone());
            let _ = connection.game_id.insert(game_state.game_id.clone());
            let mut write_game_codes = game_codes.write().await;
            write_game_codes.insert(game_state.game_code.clone(), game_state.game_id.clone());

            let game_created = GameCreated {
                game_code: game_state.game_code.clone(),
                game_id: game_state.game_id.clone(),
            };

            // these must be dropped, or else deadlock occurs, because send_ws_message_to_user
            // waits for read access, which is never given while these variables have
            // write access--which only get dropped once message is sent, and so on
            drop(write_games);
            drop(write_connections);
            drop(write_game_codes);

            // send updated new game state to owner only
            // --no need to iterate through participants, since it's a new game
            eprint!("New game successfully created! {:#?}\n", &game_state);
            // Game Created event
            send_ws_message_to_user(
                &user_id,
                STCMsg::GameCreated(game_created),
                &connections,
                &games,
                &game_codes,
            )
            .await;
            // Updated Game State
            send_ws_message_to_user(
                &user_id,
                STCMsg::GameState(Some(game_state.clone())),
                &connections,
                &games,
                &game_codes,
            )
            .await;
        }
        CTSMsg::JoinGameWithGameCode(join_game_with_game_code) => {
            let JoinGameWithGameCode {
                user_id,
                display_name,
                game_code,
            } = join_game_with_game_code;

            // Verify that user isn't already associated with another game first
            let mut write_connections = connections.write().await;
            let connection = write_connections
                .get_mut(&user_id)
                .expect(USER_ID_NOT_IN_MAP);

            // user already associated with a game, no action needed
            if let Some(game_id) = &connection.game_id {
                eprint!(
                    "Can't Join game with game code for user {}: user is already associated with a game: {}\n",
                    user_id,
                    game_id
                );
                return;
            }

            // get game_id from game_code
            let read_game_codes = game_codes.read().await;
            let game_id = read_game_codes.get(&game_code.to_uppercase());
            let cloned_gamed_id = match game_id {
                None => {
                    eprint!("User supplied incorrect game_code: ignoring request to join\n");
                    return;
                }
                Some(game_id) => game_id.clone(),
            };
            drop(read_game_codes);

            let mut write_games = games.write().await;
            let game_state_clone = write_games
                .get_mut(&cloned_gamed_id)
                .expect(GAME_ID_NOT_IN_MAP);
            let new_game_state = game_state_clone.add_user(user_id.clone(), display_name);

            // save new game state
            write_games.insert(cloned_gamed_id.clone(), new_game_state.clone());

            // associate game_id to new user
            let connection = write_connections
                .get_mut(&user_id)
                .expect(USER_ID_NOT_IN_MAP);
            let _ = connection.game_id.insert(new_game_state.game_id.clone());

            // these must be dropped, or else deadlock occurs, because send_ws_message_to_all_participants
            // waits for read access, which is never given while these variables have
            // write access--which only get dropped once message is sent, and so on
            drop(write_games);
            drop(write_connections);

            eprint!("User successfully joined game! {:#?}\n", &new_game_state);

            // Send updates to user
            // User Joined event
            send_ws_message_to_all_participants(
                &cloned_gamed_id,
                STCMsg::UserJoined(user_id.clone()),
                &connections,
                &games,
                &game_codes,
            )
            .await;

            // Game Stage Changed event
            if let GameStage::Teams = new_game_state.stage {
                send_ws_message_to_all_participants(
                    &cloned_gamed_id,
                    STCMsg::GameStageChanged(new_game_state.stage.clone()),
                    &connections,
                    &games,
                    &game_codes,
                )
                .await;
            }

            // Game State
            send_ws_message_to_all_participants(
                &cloned_gamed_id,
                STCMsg::GameState(Some(new_game_state.clone())),
                &connections,
                &games,
                &game_codes,
            )
            .await;
        }
        CTSMsg::LeaveGame => {
            let mut write_connections = connections.write().await;
            let mut write_games = games.write().await;
            let mut write_game_codes = game_codes.write().await;

            // extract game_id
            // if user is not in a game, no action needed
            let game_id_clone = match &write_connections
                .get(&user_id)
                .expect(USER_ID_NOT_IN_MAP)
                .game_id
            {
                Some(game_id) => game_id.clone(),
                None => return,
            };

            // extract all needed game state
            let game_state_clone = write_games
                .get(&game_id_clone)
                .expect(GAME_ID_NOT_IN_MAP)
                .clone();
            let game_code_clone = game_state_clone.game_code.clone();
            let participants_clone = game_state_clone.participants.clone();

            // check if any other participants are still in room
            let mut any_other_user_is_still_in_game = false;
            for participant in participants_clone.iter() {
                if participant.user_id != user_id {
                    // should always connected in lobby (since users are just removed when they disconnect)
                    // but it doesn't hurt to check
                    let participant_connection = write_connections
                        .get(&participant.user_id)
                        .expect(USER_ID_NOT_IN_MAP);
                    if participant_connection.connected {
                        any_other_user_is_still_in_game = true;
                    }
                }
            }

            // other users still in game:
            if any_other_user_is_still_in_game {
                if let GameStage::Lobby = game_state_clone.stage {
                    // if this is the lobby, remove user from the lobby, but keep connection open
                    eprint!("Removing user {} from lobby on leave game event, but keeping connection open\n", user_id);
                    let mut owner_reassigned = false;

                    // disassociate user_id with game
                    write_connections
                        .get_mut(&user_id)
                        .expect(USER_ID_NOT_IN_MAP)
                        .game_id = None;

                    // update game state by removing user and reassigning owner if needed
                    let new_game_state = if game_state_clone.owner_id == *user_id {
                        // if owner leaves in lobby, assign ownership to next participant
                        eprintln!("Reassigning owner role to a different user");
                        owner_reassigned = true;
                        game_state_clone.remove_user(&user_id).reassign_owner()
                    } else {
                        // if not the owner, just remove from state
                        game_state_clone.remove_user(&user_id)
                    };
                    *write_games
                        .get_mut(&game_code_clone)
                        .expect(GAME_ID_NOT_IN_MAP) = new_game_state.clone();

                    drop(write_connections);
                    drop(write_games);
                    drop(write_game_codes);

                    // notify remaining participants that user left
                    send_ws_message_to_all_participants(
                        &game_id_clone,
                        STCMsg::UserLeft(user_id.to_string()),
                        connections,
                        games,
                        game_codes,
                    )
                    .await;

                    // notify remaining participants that new owner was chosen
                    if owner_reassigned {
                        send_ws_message_to_all_participants(
                            &game_id_clone,
                            STCMsg::OwnerReassigned(new_game_state.owner_id.clone()),
                            connections,
                            games,
                            game_codes,
                        )
                        .await;
                    }

                    // send updated game state to other participants
                    send_ws_message_to_all_participants(
                        &game_id_clone,
                        STCMsg::GameState(Some(new_game_state)),
                        connections,
                        games,
                        game_codes,
                    )
                    .await;

                    // send a None game state to current user
                    send_ws_message_to_user(
                        &user_id,
                        STCMsg::GameState(None),
                        connections,
                        games,
                        game_codes,
                    )
                    .await;
                } else {
                    // user not in lobby: can't leave
                    eprint!(
                        "User {} can't leave game since user is not in lobby\n",
                        user_id
                    );
                    return;
                }
            } else {
                // no other users left in game: delete game but keep user connection
                eprint!(
                    "Removing user {} from game state and deleting game {}\n",
                    user_id, game_id_clone
                );

                // disassociate user with game_id
                write_connections
                    .get_mut(&user_id)
                    .expect(USER_ID_NOT_IN_MAP)
                    .game_id = None;

                // remove game from memory
                write_games.remove(&game_id_clone);
                write_game_codes.remove(&game_code_clone);

                drop(write_connections);
                drop(write_games);
                drop(write_game_codes);

                // send a None game state to current user
                send_ws_message_to_user(
                    &user_id,
                    STCMsg::GameState(None),
                    connections,
                    games,
                    game_codes,
                )
                .await;
            }
        }
        any_other_message => {
            eprint!("Unexpected message received: {:?}\n", any_other_message);

            // let user know something weird was received
            send_ws_message_to_user(
                &user_id,
                STCMsg::UnexpectedMessageReceived(format!("{:#?}", &any_other_message)),
                &connections,
                &games,
                &game_codes,
            )
            .await;
        }
    }
}

pub async fn send_ws_message_to_user(
    user_id: &String,
    msg: STCMsg,
    connections: &Connections,
    _: &Games,
    _: &GameCodes,
) {
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

pub async fn send_ws_message_to_all_participants(
    game_id: &String,
    msg: STCMsg,
    connections: &Connections,
    games: &Games,
    _: &GameCodes,
) {
    let msg = bincode::serialize(&msg).expect("Could not serialize message");
    let msg = Message::binary(msg);
    let games = games.read().await;
    let game = games.get(game_id).expect(GAME_ID_NOT_IN_MAP);
    for participant in game.participants.iter() {
        let read_connections = connections.read().await;
        let ws = read_connections
            .get(&participant.user_id)
            .expect(USER_ID_NOT_IN_MAP);
        if let Err(_disconnected) = ws.tx.send(msg.clone()) {
            eprint!("User is disconnected. Couldn't send message {:?}\n", &msg);
        } else {
            eprint!("Message successfully sent\n");
        }
    }
}

pub async fn cleanup_state_after_disconnect(
    user_id: &String,
    connections: &Connections,
    games: &Games,
    game_codes: &GameCodes,
) {
    eprintln!("User disconnected: {}\n", user_id);

    // extract game_id
    let mut write_connections = connections.write().await;
    let mut write_games = games.write().await;
    let mut write_game_codes = game_codes.write().await;
    let game_id_clone = &write_connections
        .get(user_id)
        .expect(USER_ID_NOT_IN_MAP)
        .game_id
        .clone();

    match game_id_clone {
        // user is associated with a game_id (should always be the case)
        Some(game_id) => {
            // extract all needed game state
            let game_state_clone = write_games.get(game_id).expect(GAME_ID_NOT_IN_MAP).clone();
            let game_code_clone = game_state_clone.game_code.clone();
            let participants_clone = game_state_clone.participants.clone();

            // check if any other participants are still connected
            let mut any_other_user_is_still_in_game = false;
            for participant in participants_clone.iter() {
                if &participant.user_id != user_id {
                    let participant_connection = write_connections
                        .get(&participant.user_id)
                        .expect(USER_ID_NOT_IN_MAP);
                    if participant_connection.connected {
                        any_other_user_is_still_in_game = true;
                    }
                }
            }

            // other users still in game:
            if any_other_user_is_still_in_game {
                if let GameStage::Lobby = game_state_clone.stage {
                    // if this is the lobby, remove from state
                    eprint!("Removing user {} from lobby on disconnect\n", user_id);
                    let mut owner_reassigned = false;
                    write_connections.remove(user_id);
                    let new_game_state = if game_state_clone.owner_id == *user_id {
                        // if owner leaves in lobby, assign ownership to next participant
                        eprintln!("Reassigning owner role to a different user");
                        owner_reassigned = true;
                        game_state_clone.remove_user(user_id).reassign_owner()
                    } else {
                        // if not the owner, just remove from state
                        game_state_clone.remove_user(user_id)
                    };

                    *write_games.get_mut(game_id).expect(GAME_ID_NOT_IN_MAP) =
                        new_game_state.clone();

                    drop(write_connections);
                    drop(write_games);
                    drop(write_game_codes);

                    // notify remaining participants that user left
                    send_ws_message_to_all_participants(
                        game_id,
                        STCMsg::UserLeft(user_id.to_string()),
                        connections,
                        games,
                        game_codes,
                    )
                    .await;

                    // notify remaining participants that new owner was chosen
                    if owner_reassigned {
                        send_ws_message_to_all_participants(
                            game_id,
                            STCMsg::OwnerReassigned(new_game_state.owner_id.clone()),
                            connections,
                            games,
                            game_codes,
                        )
                        .await;
                    }

                    // send updated game state
                    send_ws_message_to_all_participants(
                        game_id,
                        STCMsg::GameState(Some(new_game_state)),
                        connections,
                        games,
                        game_codes,
                    )
                    .await;
                } else {
                    // else only mark this user as disconnected
                    eprint!("Marking user {} as not connected\n", user_id);
                    write_connections
                        .get_mut(user_id)
                        .expect(USER_ID_NOT_IN_MAP)
                        .connected = false;

                    drop(write_connections);
                    drop(write_games);
                    drop(write_game_codes);

                    // notify remaining participants that user was disconnected
                    send_ws_message_to_all_participants(
                        game_id,
                        STCMsg::UserDisconnected(user_id.to_string()),
                        connections,
                        games,
                        game_codes,
                    )
                    .await;

                    // send old game state (no change occurred)
                    send_ws_message_to_all_participants(
                        game_id,
                        STCMsg::GameState(Some(game_state_clone)),
                        connections,
                        games,
                        game_codes,
                    )
                    .await;
                }
            } else {
                // no users left in game: remove all users from Connections and delete game from Games
                eprint!(
                    "Removing all users and game from state for game {}\n",
                    game_id
                );
                for participant in participants_clone.iter() {
                    write_connections.remove(&participant.user_id);
                }
                write_games.remove(game_id);
                write_game_codes.remove(&game_code_clone);
            }
        }
        None => {
            // user was not associated with any game (shouldn't happen), remove from Connections
            write_connections.remove(user_id);
        }
    }
}
