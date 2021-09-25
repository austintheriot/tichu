use common::{validate_display_name, validate_game_code, GameStage, JoinGameWithGameCode, STCMsg};

use crate::{
    errors::USER_ID_NOT_IN_MAP,
    routes::ws::{send_game_state_to_all_participants, send_ws_message_to_all_participants},
    Connections, GameCodes, Games,
};

pub async fn join_game_with_game_code(
    join_game_with_game_code_data: JoinGameWithGameCode,
    connections: &Connections,
    games: &Games,
    game_codes: &GameCodes,
) {
    let JoinGameWithGameCode {
        user_id,
        display_name,
        game_code,
    } = join_game_with_game_code_data;

    // bad inputs from client, ignore request
    if validate_display_name(&display_name).is_some() || validate_game_code(&game_code).is_some() {
        return;
    }

    // Verify that user isn't already associated with another game first
    let mut write_connections = connections.write().await;
    let connection = write_connections
        .get_mut(&user_id)
        .expect(USER_ID_NOT_IN_MAP);

    // user already associated with a game, no action needed
    if let Some(game_id) = &connection.game_id {
        eprintln!(
    "Can't Join game with game code for user {}: user is already associated with a game: {}",
    user_id,
    game_id
  );
        return;
    }

    // get game_id from game_code
    let read_game_codes = game_codes.read().await;
    let game_id = read_game_codes.get(&game_code.to_uppercase());
    let cloned_gamed_id = match game_id {
        None => {
            eprintln!("User supplied incorrect game_code: ignoring request to join");
            return;
        }
        Some(game_id) => game_id.clone(),
    };
    drop(read_game_codes);

    let mut write_games = games.write().await;
    let game_state_clone = write_games
        .get_mut(&cloned_gamed_id)
        .expect(USER_ID_NOT_IN_MAP);

    // Verify that there are not already 4 users in the game
    if game_state_clone.participants.len() == 4 {
        eprintln!(
            "There are already 4 users in game {}: ignoring request to join from user {}",
            cloned_gamed_id, user_id
        );
        return;
    }

    let new_game_state = game_state_clone.add_user(user_id.clone(), display_name);

    // save new game state
    write_games.insert(cloned_gamed_id.clone(), new_game_state.clone());

    // associate game_id to new user
    let connection = write_connections
        .get_mut(&user_id)
        .expect(USER_ID_NOT_IN_MAP);
    let _ = connection.game_id.insert(new_game_state.game_id.clone());

    // these must be dropped, or else deadlock occurs, because send_ws_message_to_all_participants
    // waits for read access, which is never given while these variables have
    // write access--which only get dropped once message is sent, and so on
    drop(write_games);
    drop(write_connections);

    eprintln!("User successfully joined game! {:#?}", &new_game_state);

    // Send updates to user
    // User Joined event
    send_ws_message_to_all_participants(
        &cloned_gamed_id,
        STCMsg::UserJoined(user_id.clone()),
        &connections,
        &games,
        &game_codes,
    )
    .await;

    // Game Stage Changed event
    if let GameStage::Teams(_) = new_game_state.stage {
        send_ws_message_to_all_participants(
            &cloned_gamed_id,
            STCMsg::GameStageChanged(new_game_state.stage.clone()),
            &connections,
            &games,
            &game_codes,
        )
        .await;
    }

    // Game State
    send_game_state_to_all_participants(
        &cloned_gamed_id,
        &new_game_state,
        &connections,
        &games,
        &game_codes,
    )
    .await;
}
