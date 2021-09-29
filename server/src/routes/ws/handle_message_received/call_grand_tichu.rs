use crate::{routes::ws::send_ws_message, Connections, GameCodes, Games};
use common::{PrivateGameStage, STCMsg, TichuCallStatus};

pub async fn call_grand_tichu(
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

    match &game_state.stage {
        PrivateGameStage::PrivateGrandTichu(grand_tichu_state) => {
            // player must be undecided
            let i = grand_tichu_state
                .grand_tichus
                .iter()
                .position(|user_call_status| *user_call_status.user_id == *user_id);
            match i {
                None => {
                    eprintln!("Couldn't find user's call status in GrandTichu call stage. Ignoring request to call Grand Tichu from user {}", user_id);
                    return;
                }
                Some(i) => {
                    if grand_tichu_state.grand_tichus[i].tichu_call_status
                        != TichuCallStatus::Undecided
                    {
                        eprintln!("User has already declared or declined Grand Tichu. Ignoring request to call Grand Tichu from user {}", user_id);
                        return;
                    }
                }
            }
        }
        // game stage must be GrandTichu
        _ => {
            eprintln!("Must be in GrandTichu game stage to call Grand tichu. Ignoring request to call Grand Tichu from user {}", user_id);
            return;
        }
    }

    // update game state
    let new_game_state = game_state.call_grand_tichu(user_id);
    *game_state = new_game_state.clone();

    drop(write_games);

    // send GrandTichuCalled
    send_ws_message::to_group(
        &game_id,
        STCMsg::GrandTichuCalled(user_id.to_string()),
        connections,
        games,
        game_codes,
    )
    .await;

    // send updated game state
    send_ws_message::game_state_to_group(&game_id, &new_game_state, connections, games, game_codes)
        .await;
}
