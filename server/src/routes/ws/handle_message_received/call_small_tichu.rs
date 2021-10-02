use crate::{routes::ws::send_ws_message, Connections, GameCodes, Games};
use common::{PrivateGameStage, STCMsg, TichuCallStatus};

pub async fn call_small_tichu(
    user_id: &str,
    connections: &Connections,
    games: &Games,
    game_codes: &GameCodes,
) {
    let read_connections = connections.read().await;
    let mut write_games = games.write().await;

    let user = match read_connections.get(user_id) {
        Some(user) => user,
        None => return,
    };

    let game_id = match user.game_id.clone() {
        Some(game_id) => game_id,
        None => return,
    };

    let game_state = match write_games.get_mut(&game_id) {
        Some(game_state) => game_state,
        None => return,
    };

    // game stage cannot be lobby, teams, or scoreboard
    let small_tichus = match &mut game_state.stage {
        PrivateGameStage::Lobby | PrivateGameStage::Teams(_) | PrivateGameStage::Scoreboard => {
            eprintln!(
                "Can't call Small Tichu when game is not active. Ignoring request from user {}",
                user_id
            );
            return;
        }
        PrivateGameStage::GrandTichu(grand_tichu_state) => &mut grand_tichu_state.small_tichus,
        // TODO: add other game stages here
        _ => {
            eprintln!("Can't call Small Tichu when game stage is not GrandTichu (TODO: update once other stages are implemented). Ignoring request from user {}", user_id);
            return;
        }
    };

    let i = small_tichus
        .iter()
        .position(|user_call_status| *user_call_status.user_id == *user_id);
    match i {
        None => {
            eprintln!("Couldn't find user's call status in call stage state. Ignoring request to call Small Tichu from user {}", user_id);
            return;
        }
        Some(i) => {
            let user_call_status = &small_tichus[i];
            if user_call_status.tichu_call_status != TichuCallStatus::Undecided {
                eprintln!("User is not in Undecided state about Small Tichu. Ignoring request to call Small Tichu from user {}", user_id);
                return;
            }
        }
    }

    // update game state
    let new_game_state = game_state.call_small_tichu(user_id);
    *game_state = new_game_state.clone();

    drop(write_games);

    // send GrandTichuCalled
    send_ws_message::to_group(
        &game_id,
        STCMsg::SmallTichuCalled(user_id.to_string()),
        connections,
        games,
        game_codes,
    )
    .await;

    // send updated game state
    send_ws_message::game_state_to_group(&game_id, &new_game_state, connections, games, game_codes)
        .await;
}
