use crate::{routes::ws::send_ws_message, Connections, GameCodes, Games};
use common::{PrivateGameStage, STCMsg, TichuCallStatus};

const FUNCTION_NAME: &str = "call_small_tichu";

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
        None => {
            eprintln!("{FUNCTION_NAME}: User {user_id} can't call Small Tichu because their user_id could not be found in the Connections HashMap");
            return;
        }
    };

    let game_id = match user.game_id.clone() {
        Some(game_id) => game_id,
        None => {
            eprintln!("{FUNCTION_NAME}: User {user_id} can't call Small Tichu because they have no game_id associated with their Connection state");
            return;
        }
    };

    let game_state = match write_games.get_mut(&game_id) {
        Some(game_state) => game_state,
        None => {
            eprintln!("{FUNCTION_NAME}: User {user_id} can't call Small Tichu because the game_id they are associated with could not be found in the Games HashMap");
            return;
        }
    };

    // game stage cannot be lobby, teams, or scoreboard
    let small_tichus = match &mut game_state.stage {
        PrivateGameStage::Lobby | PrivateGameStage::Teams(_) | PrivateGameStage::Scoreboard => {
            eprintln!(
                "{FUNCTION_NAME}: Can't call Small Tichu when game is not active. Ignoring request from user {}",
                user_id
            );
            return;
        }
        PrivateGameStage::GrandTichu(grand_tichu_state) => &mut grand_tichu_state.small_tichus,
        PrivateGameStage::Trade(trade) => &mut trade.small_tichus,
        PrivateGameStage::Play(play) => &mut play.small_tichus,
        _ => {
            eprintln!("{FUNCTION_NAME}: Can't call Small Tichu when game stage is not an active game stage. Ignoring request from user {}", user_id);
            return;
        }
    };

    let i = small_tichus
        .iter()
        .position(|user_call_status| *user_call_status.user_id == *user_id);
    match i {
        None => {
            eprintln!("{FUNCTION_NAME}: Couldn't find user's call status in call stage state. Ignoring request to call Small Tichu from user {}", user_id);
            return;
        }
        Some(i) => {
            let user_call_status = &small_tichus[i];
            if user_call_status.tichu_call_status != TichuCallStatus::Undecided {
                eprintln!("{FUNCTION_NAME}: User is not in Undecided state about Small Tichu. Ignoring request to call Small Tichu from user {}", user_id);
                return;
            }
        }
    }

    eprintln!(
        "{FUNCTION_NAME}: Small Tichu successfully called by user {}",
        user_id
    );

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
