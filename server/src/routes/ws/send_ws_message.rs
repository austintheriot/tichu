use crate::{
    errors::{GAME_ID_NOT_IN_MAP, USER_ID_NOT_IN_MAP},
    Connections, GameCodes, Games,
};
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
    let game = read_games.get(game_id).expect(USER_ID_NOT_IN_MAP);
    for participant in game.participants.iter() {
        // format state for this user
        let public_game_state = private_game_state.to_public_game_state(&participant.user_id);
        let msg = bincode::serialize(&STCMsg::GameState(Box::new(public_game_state)))
            .expect("Could not serialize message");
        let msg = Message::binary(msg);

        let read_connections = connections.read().await;
        let ws = read_connections
            .get(&participant.user_id)
            .expect(USER_ID_NOT_IN_MAP);
        if let Err(_disconnected) = ws.tx.send(msg.clone()) {
            eprintln!("User is disconnected. Couldn't send message {:?}", &msg);
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
    let msg = bincode::serialize(&stc_msg).expect("Could not serialize message");
    let msg = Message::binary(msg);
    let read_games = games.read().await;
    let game = read_games.get(game_id).expect(GAME_ID_NOT_IN_MAP);
    for participant in game.participants.iter() {
        let read_connections = connections.read().await;
        let ws = read_connections
            .get(&participant.user_id)
            .expect(USER_ID_NOT_IN_MAP);
        if let Err(_disconnected) = ws.tx.send(msg.clone()) {
            eprintln!("User is disconnected. Couldn't send message {:?}", &msg);
        } else {
            eprintln!("Message successfully sent");
        }
    }
}

/// Sends a single server-to-client websocket message to a single participant.
pub async fn to_user(user_id: &str, msg: STCMsg, connections: &Connections) {
    let msg = bincode::serialize(&msg).expect("Could not serialize message");
    let msg = Message::binary(msg);
    let read_connections = connections.read().await;
    let ws = read_connections.get(user_id).expect(USER_ID_NOT_IN_MAP);
    if let Err(_disconnected) = ws.tx.send(msg.clone()) {
        eprintln!("User is disconnected. Couldn't send message {:?}", &msg);
    } else {
        eprintln!("Message successfully sent");
    }
}
