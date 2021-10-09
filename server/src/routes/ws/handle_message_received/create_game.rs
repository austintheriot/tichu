use super::send_ws_message;
use crate::{errors::USER_ID_NOT_IN_MAP, Connections, GameCodes, Games};
use common::{validate_display_name, CTSMsg, PrivateGameState, STCMsg};

pub async fn create_game(
    create_game_data: CTSMsg,
    connections: Connections,
    games: &Games,
    game_codes: &GameCodes,
) {
    if let CTSMsg::CreateGame {
        user_id,
        display_name,
    } = create_game_data
    {
        // bad inputs from client, ignore request
        if validate_display_name(&display_name).is_some() {
            return;
        }

        // verify that user_id is not already associated with a game before creating a new one
        let mut write_connections = connections.write().await;
        let connection = write_connections
            .get_mut(&user_id)
            .expect(USER_ID_NOT_IN_MAP);

        // user already associated with a game, no action needed
        if let Some(game_id) = &connection.game_id {
            eprintln!(
                "create_game: Can't create game for user: user is already associated with a game: {}",
                game_id
            );
            return;
        }

        // user is NOT associated with a game: create game for user
        let read_game_codes = game_codes.read().await;
        let game_state = PrivateGameState::new(user_id.clone(), display_name, &read_game_codes);
        drop(read_game_codes);

        // save game to state
        let mut write_games = games.write().await;
        write_games.insert(game_state.game_id.clone(), game_state.clone());
        let _ = connection.game_id.insert(game_state.game_id.clone());
        let mut write_game_codes = game_codes.write().await;
        write_game_codes.insert(game_state.game_code.clone(), game_state.game_id.clone());

        let game_created = STCMsg::GameCreated {
            game_code: game_state.game_code.clone(),
            game_id: game_state.game_id.clone(),
        };

        // these must be dropped, or else deadlock occurs, because send_ws_message_to_user
        // waits for read access, which is never given while these variables have
        // write access--which only get dropped once message is sent, and so on
        drop(write_games);
        drop(write_connections);
        drop(write_game_codes);

        // send updated new game state to owner only
        // --no need to iterate through participants, since it's a new game
        eprintln!("New game successfully created! {:#?}", &game_state);
        // Game Created event
        send_ws_message::to_user(&user_id, game_created, &connections).await;

        // Updated Game State
        send_ws_message::to_user(
            &user_id,
            STCMsg::GameState(Box::new(game_state.to_public_game_state(&user_id))),
            &connections,
        )
        .await;
    }
}
