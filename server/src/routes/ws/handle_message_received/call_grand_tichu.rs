use crate::{routes::ws::send_ws_message, Connections, GameCodes, Games};
use common::{CallGrandTichuRequest, PrivateGameStage, STCMsg, TichuCallStatus};

const FUNCTION_NAME: &str = "call_grand_tichu";

pub async fn call_grand_tichu(
    call_grand_tichu_request: &CallGrandTichuRequest,
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
            eprintln!("{FUNCTION_NAME}: User {user_id} can't call Grand Tichu, because their user_id could not be found in the Connections HashMap");
            return;
        }
    };

    let game_id = match user.game_id.clone() {
        Some(game_id) => game_id,
        None => {
            eprintln!("{FUNCTION_NAME}: User {user_id} can't call Grand Tichu, because the user is not associated with a game_id");
            return;
        }
    };

    let game_state = match write_games.get_mut(&game_id) {
        Some(game_state) => game_state,
        None => {
            eprintln!("{FUNCTION_NAME}: User {user_id} can't call Grand Tichu, because the game_id that they are associated with could not be found in the Games HashMap");
            return;
        }
    };

    match &game_state.stage {
        PrivateGameStage::GrandTichu(grand_tichu_state) => {
            // player must be undecided
            let i = grand_tichu_state
                .grand_tichus
                .iter()
                .position(|user_call_status| *user_call_status.user_id == *user_id);
            match i {
                None => {
                    eprintln!("{FUNCTION_NAME}: User {user_id} can't call Grand Tichu their call status in GrandTichu call stage couldn't be found");
                    return;
                }
                Some(i) => {
                    if grand_tichu_state.grand_tichus[i].tichu_call_status
                        != TichuCallStatus::Undecided
                    {
                        eprintln!("{FUNCTION_NAME}: User {user_id} can't call Grand Tichu because they have already declared or declined Grand Tichu");
                        return;
                    }
                }
            }
        }
        // game stage must be GrandTichu
        _ => {
            eprintln!("{FUNCTION_NAME}: User {user_id} can't call Grand Tichu because they are not in the Grand Tichu game stage");
            return;
        }
    }

    // update game state
    let new_game_state = match game_state.call_grand_tichu(call_grand_tichu_request, user_id) {
        Ok(new_game_state) => new_game_state,
        Err(error) => return eprintln!("{}", error),
    };

    *game_state = new_game_state.clone();

    drop(write_games);

    eprintln!(
        "User {user_id} successfully called Grand Tichu as {:#?}",
        call_grand_tichu_request
    );

    // send GrandTichuCalled
    send_ws_message::to_group(
        &game_id,
        STCMsg::GrandTichuCalled(user_id.to_string(), call_grand_tichu_request.clone()),
        connections,
        games,
        game_codes,
    )
    .await;

    // if game stage changed to Trade, send GameStageChanged event
    if let PrivateGameStage::Trade(_) = &new_game_state.stage {
        send_ws_message::to_group(
            &game_id,
            STCMsg::LastCardsDealt,
            connections,
            games,
            game_codes,
        )
        .await;

        send_ws_message::to_group(
            &game_id,
            STCMsg::GameStageChanged(new_game_state.stage.clone().into()),
            connections,
            games,
            game_codes,
        )
        .await;
    }

    // send updated game state
    send_ws_message::game_state_to_group(&game_id, &new_game_state, connections, games, game_codes)
        .await;
}
