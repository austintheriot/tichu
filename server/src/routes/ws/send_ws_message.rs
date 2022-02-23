use crate::{Connections, GameCodes, Games};
use common::{PrivateGameState, STCMsg};
use warp::ws::Message;

/// Ensures that each user receives a version of the state that only THEY are allowed to see.
/// I.e. each user can see everything in the state except for the other user's cards, etc.
pub async fn game_state_to_group(
    game_id: &str,
    private_game_state: &PrivateGameState,
    connections: &Connections,
    games: &Games,
    _: &GameCodes,
) {
    let read_games = games.read().await;
    let game = if let Some(game_state) = read_games.get(game_id) {
        game_state.clone()
    } else {
        eprintln!("game_id {} not found in read_games", game_id);
        return;
    };
    drop(read_games);

    for participant in game.participants.iter() {
        // format state for this user
        let public_game_state = private_game_state.to_public_game_state(&participant.user_id);

        let msg = if let Ok(msg) =
            bincode::serialize(&STCMsg::GameState(Box::new(public_game_state.ok())))
        {
            msg
        } else {
            eprintln!("Could not serialize message");
            continue;
        };

        let msg = Message::binary(msg);

        let read_connections = connections.read().await;
        let connection_data = match read_connections.get(&participant.user_id) {
            Some(participant_connection) => participant_connection,
            None => {
                eprintln!("Can't send game state to user {}, because user was not found in Connections HahsMap", participant.user_id);
                continue;
            }
        };
        if let Err(_disconnected) = connection_data.tx.send(msg.clone()) {
            eprintln!("User is disconnected. Couldn't send message.");
        } else {
            eprintln!("Message successfully sent");
        }
    }
}

/// Sends any server-to-client websocket message to all participants in the game represented by the given game_id.
pub async fn to_group(
    game_id: &str,
    stc_msg: STCMsg,
    connections: &Connections,
    games: &Games,
    _: &GameCodes,
) {
    let msg = if let Ok(msg) = bincode::serialize(&stc_msg) {
        msg
    } else {
        eprintln!("Could not serialize message {:?}", stc_msg);
        return;
    };

    let msg = Message::binary(msg);
    let read_games = games.read().await;
    let game_state = if let Some(game_state) = read_games.get(game_id) {
        game_state
    } else {
        eprintln!("Could not find game {} in game_state", game_id);
        return;
    };

    for participant in game_state.participants.iter() {
        let read_connections = connections.read().await;
        let ws = match read_connections.get(&participant.user_id) {
            Some(participant_connection) => participant_connection,
            // participant not found in Connections map
            None => {
                eprintln!("Can't send game state to user {}, because user was not found in Connections HahsMap", participant.user_id);
                continue;
            }
        };
        if let Err(_disconnected) = ws.tx.send(msg.clone()) {
            eprintln!("User is disconnected. Couldn't send message.");
        } else {
            eprintln!("Message successfully sent");
        }
    }
}

/// Sends a single server-to-client websocket message to a single participant.
pub async fn to_user(user_id: &str, msg: STCMsg, connections: &Connections) {
    let msg = if let Ok(msg) = bincode::serialize(&msg) {
        msg
    } else {
        eprintln!("Could not serialize websocket message {:?}", msg);
        return;
    };
    let msg = Message::binary(msg);
    let read_connections = connections.read().await;
    let connection_data = if let Some(connection_data) = read_connections.get(user_id) {
        connection_data
    } else {
        eprintln!("Could not find connection data for user {}", user_id);
        return;
    };
    if let Err(_disconnected) = connection_data.tx.send(msg.clone()) {
        eprintln!("User is disconnected. Couldn't send message.");
    } else {
        eprintln!("Message successfully sent");
    }
}
