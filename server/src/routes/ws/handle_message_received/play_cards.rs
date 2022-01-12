use crate::{routes::ws::send_ws_message, Connections, GameCodes, Games};
use common::{Card, CardValue, PrivateGameStage, STCMsg};

const FUNCTION_NAME: &str = "play_cards";

pub async fn play_cards(
    user_id: &str,
    cards: Vec<Card>,
    wished_for: Option<CardValue>,
    user_id_to_give_dragon_to: Option<String>,
    connections: &Connections,
    games: &Games,
    game_codes: &GameCodes,
) {
    let read_connections = connections.read().await;
    let mut write_games = games.write().await;

    let user = match read_connections.get(user_id) {
        Some(user) => user,
        None => {
            eprintln!("{FUNCTION_NAME}: User {} can't play cards, because their user_id could not be found in the Connections HashMap", user_id);
            return;
        }
    };

    let game_id = match user.game_id.clone() {
        Some(game_id) => game_id,
        None => {
            eprintln!("{FUNCTION_NAME}: User {} can't play cards, because they are not associated with a game_id", user_id);
            return;
        }
    };

    let game_state = match write_games.get_mut(&game_id) {
        Some(game_state) => game_state,
        None => {
            eprintln!("{FUNCTION_NAME}: User {} can't play cards, because the game they are associated with could not be found in the Games HashMap", user_id);
            return;
        }
    };

    // update game state
    let new_game_state =
        game_state.play_cards(user_id, cards, wished_for, user_id_to_give_dragon_to);
    let new_game_state = match new_game_state {
        Ok(new_game_state) => new_game_state,
        Err(error) => {
            eprintln!("{}", error);
            return;
        }
    };
    *game_state = new_game_state.clone();

    drop(write_games);

    eprintln!(
        "{FUNCTION_NAME}: User {} successfully played cards",
        user_id
    );

    // send Cards played event
    send_ws_message::to_group(
        &game_id,
        STCMsg::CardsPlayed,
        connections,
        games,
        game_codes,
    )
    .await;

    // send updated game state
    send_ws_message::game_state_to_group(&game_id, &new_game_state, connections, games, game_codes)
        .await;
}
