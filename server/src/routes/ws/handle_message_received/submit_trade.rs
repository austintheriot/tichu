use crate::{routes::ws::send_ws_message, Connections, GameCodes, Games};
use common::{PrivateGameStage, STCMsg, SubmitTrade};

const FUNCTION_NAME: &str = "submit_trade";

pub async fn submit_trade(
    trade_array: SubmitTrade,
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
    let new_game_state = game_state.submit_trade(user_id, &trade_array);
    *game_state = new_game_state.clone();

    drop(write_games);

    eprintln!(
        "{FUNCTION_NAME}: User {} successfully submitted trade",
        user_id
    );

    // send Trade submitted
    send_ws_message::to_group(
        &game_id,
        STCMsg::TradeSubmitted(user_id.to_string()),
        connections,
        games,
        game_codes,
    )
    .await;

    // if game stage changed to Play, send GameStageChanged event
    if let PrivateGameStage::Play(_) = &new_game_state.stage {
        eprintln!(
            "{FUNCTION_NAME}: Final trade submitted. Game state moved to Play for game {game_id}"
        );
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
