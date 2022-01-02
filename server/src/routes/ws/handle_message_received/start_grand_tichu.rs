use crate::{
    errors::{GAME_ID_NOT_IN_MAP, USER_ID_NOT_IN_MAP},
    routes::ws::send_ws_message,
    Connections, GameCodes, Games,
};
use common::{PrivateGameStage, STCMsg};

const FUNCTION_NAME: &str = "start_grand_tichu";

pub async fn start_grand_tichu(
    user_id: &str,
    connections: &Connections,
    games: &Games,
    game_codes: &GameCodes,
) {
    // requesting user must be owner
    let read_connections = connections.read().await;
    let mut write_games = games.write().await;

    let user = read_connections.get(user_id).expect(USER_ID_NOT_IN_MAP);
    let game_id_clone = user.game_id.clone();

    let game_id_clone = match &game_id_clone {
        None => {
            eprintln!("{FUNCTION_NAME}: User {} can't start game since they are not associated with any game. Ignoring request", user_id);
            return;
        }
        Some(game_id_clone) => game_id_clone,
    };

    let game_state = write_games
        .get_mut(game_id_clone)
        .expect(GAME_ID_NOT_IN_MAP);

    // game stage must be Teams
    let teams_state = match &game_state.stage {
        PrivateGameStage::Teams(teams_state) => teams_state,
        _ => {
            eprintln!("{FUNCTION_NAME}: User {} can't start game because current game stage is not teams. Ignoring request", user_id);
            return;
        }
    };

    // teams must be 2v2
    if teams_state[0].user_ids.len() != 2 || teams_state[1].user_ids.len() != 2 {
        eprintln!(
            "{FUNCTION_NAME}: User {} can't start game because teams are not even 2v2. Ignoring request",
            user_id
        );
        return;
    }

    // update game state
    let new_game_state = match game_state.start_grand_tichu(user_id) {
        Ok(updated_game_state) => updated_game_state,
        Err(err) => return eprintln!("{}", err),
    };
    *game_state = new_game_state.clone();

    drop(write_games);

    eprintln!(
        "{FUNCTION_NAME}: User {} successfully started Grand Tichu",
        user_id
    );

    // send PrivateGameStage change event to Grand Tichu
    send_ws_message::to_group(
        game_id_clone,
        STCMsg::GameStageChanged(new_game_state.stage.clone().into()),
        connections,
        games,
        game_codes,
    )
    .await;

    // send FirstCardsDealt event
    send_ws_message::to_group(
        game_id_clone,
        STCMsg::FirstCardsDealt,
        connections,
        games,
        game_codes,
    )
    .await;

    // send updated game state
    send_ws_message::game_state_to_group(
        game_id_clone,
        &new_game_state,
        connections,
        games,
        game_codes,
    )
    .await;
}
