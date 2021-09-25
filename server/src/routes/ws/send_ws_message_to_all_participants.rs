use common::STCMsg;
use warp::ws::Message;

use crate::{
    errors::{GAME_ID_NOT_IN_MAP, USER_ID_NOT_IN_MAP},
    Connections, GameCodes, Games,
};

/// Sends any server-to-client websocket message to all participants in the game represented by the given game_id.
pub async fn send_ws_message_to_all_participants(
    game_id: &String,
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
            eprint!("User is disconnected. Couldn't send message {:?}\n", &msg);
        } else {
            eprint!("Message successfully sent\n");
        }
    }
}
