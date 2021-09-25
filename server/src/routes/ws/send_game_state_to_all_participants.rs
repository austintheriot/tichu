use crate::{errors::USER_ID_NOT_IN_MAP, Connections, GameCodes, Games};
use common::{PrivateGameState, STCMsg};
use warp::ws::Message;

/// Ensures that each user receives a version of the state that only THEY are allowed to see.
/// I.e. each user can see everything in the state except for the other user's cards, etc.
pub async fn send_game_state_to_all_participants(
    game_id: &String,
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
        let msg = bincode::serialize(&STCMsg::GameState(public_game_state))
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
