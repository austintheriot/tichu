use crate::{routes::ws::send_ws_message, Connections, GameCodes, Games};

const FUNCTION_NAME: &str = "__admin__skip_to_play";

pub async fn __admin_skip_to_play(
    user_id: &str,
    connections: &Connections,
    games: &Games,
    game_codes: &GameCodes,
) {
    let read_connections = connections.read().await;
    let mut write_games = games.write().await;

    let user = match read_connections.get(user_id) {
        Some(user) => user,
        None => {
            eprintln!("{FUNCTION_NAME}: User {} can't submit trade, because their user_id could not be found in the Connections HashMap", user_id);
            return;
        }
    };

    let game_id = match user.game_id.clone() {
        Some(game_id) => game_id,
        None => {
            eprintln!("{FUNCTION_NAME}: User {} can't submit trade, because they are not associated with a game_id", user_id);
            return;
        }
    };

    let game_state = match write_games.get_mut(&game_id) {
        Some(game_state) => game_state,
        None => {
            eprintln!("{FUNCTION_NAME}: User {} can't submit trade, because the game they are associated with could not be found in the Games HashMap", user_id);
            return;
        }
    };

    // update game state
    let new_game_state = match game_state.__admin_skip_to_play() {
        Ok(updated_game_state) => updated_game_state,
        Err(err) => return eprintln!("{}", err),
    };

    *game_state = new_game_state.clone();

    drop(write_games);

    // send updated game state
    send_ws_message::game_state_to_group(&game_id, &new_game_state, connections, games, game_codes)
        .await;
}
