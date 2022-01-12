use crate::{routes::ws::send_ws_message, Connections, GameCodes, Games};
use common::STCMsg;

const FUNCTION_NAME: &str = "pass";

pub async fn pass(user_id: &str, connections: &Connections, games: &Games, game_codes: &GameCodes) {
    let read_connections = connections.read().await;
    let mut write_games = games.write().await;

    let user = match read_connections.get(user_id) {
        Some(user) => user,
        None => {
            eprintln!("{FUNCTION_NAME}: User {} can't pass, because their user_id could not be found in the Connections HashMap", user_id);
            return;
        }
    };

    let game_id = match user.game_id.clone() {
        Some(game_id) => game_id,
        None => {
            eprintln!("{FUNCTION_NAME}: User {} can't pass, because they are not associated with a game_id", user_id);
            return;
        }
    };

    let game_state = match write_games.get_mut(&game_id) {
        Some(game_state) => game_state,
        None => {
            eprintln!("{FUNCTION_NAME}: User {} can't pass, because the game they are associated with could not be found in the Games HashMap", user_id);
            return;
        }
    };

    // update game state
    let new_game_state = match game_state.pass(user_id) {
        Ok(updated_game_state) => updated_game_state,
        Err(err) => return eprintln!("{}", err),
    };
    *game_state = new_game_state.clone();

    drop(write_games);

    eprintln!("{FUNCTION_NAME}: User {} successfully passed", user_id);

    // send Pass event to group
    send_ws_message::to_group(
        &game_id,
        STCMsg::UserPassed(user_id.into()),
        connections,
        games,
        game_codes,
    )
    .await;

    // if someone won the trick, send event
    // todo!();

    // round over?
    // todo!();

    // game over?
    // todo!();

    // if game stage changed to Scoreboard, send GameStageChanged event?
    // if let PrivateGameStage::Score = &new_game_state.stage {
    //     eprintln!("{FUNCTION_NAME}: Game state moved to Scoreboard for game {game_id}");
    //     send_ws_message::to_group(
    //         &game_id,
    //         STCMsg::GameStageChanged(new_game_state.stage.clone().into()),
    //         connections,
    //         games,
    //         game_codes,
    //     )
    //     .await;
    // }

    // send updated game state
    send_ws_message::game_state_to_group(&game_id, &new_game_state, connections, games, game_codes)
        .await;
}
