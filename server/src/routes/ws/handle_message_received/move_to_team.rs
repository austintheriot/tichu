use crate::{
    errors::{GAME_ID_NOT_IN_MAP, USER_ID_NOT_IN_MAP},
    routes::ws::send_ws_message,
    Connections, GameCodes, Games,
};
use common::{GameStage, STCMsg, TeamOption};

pub async fn move_to_team(
    team_to_move_to: &TeamOption,
    user_id: &str,
    connections: &Connections,
    games: &Games,
    game_codes: &GameCodes,
) {
    let read_connections = connections.read().await;
    let mut write_games = games.write().await;

    let game_id_clone = read_connections
        .get(user_id)
        .expect(USER_ID_NOT_IN_MAP)
        .game_id
        .clone();
    let game_id_clone = match game_id_clone {
        // user is not associated with a game, do nothing
        None => {
            eprintln!(
                "User {} is not associated with a game. Ignoring request",
                &user_id
            );
            return;
        }
        Some(game_id_clone) => game_id_clone,
    };
    let prev_game_state = write_games.get(&game_id_clone).expect(GAME_ID_NOT_IN_MAP);
    match &prev_game_state.stage {
        GameStage::Teams(teams_state) => {
            let team = match &team_to_move_to {
                &TeamOption::TeamA => &teams_state[0],
                &TeamOption::TeamB => &teams_state[1],
            };
            // if user is already on the team they want to move to, ignore request
            if team
                .user_ids
                .iter()
                .find(|participant_id| **participant_id == user_id)
                .is_some()
            {
                eprintln!(
                    "User {} is already on team {:?}. Ignoring request",
                    &user_id, &team_to_move_to
                );
                return;
            }
        }
        // current stage is not Teams, do nothing
        _ => {
            eprintln!(
                "Current stage is not Teams. Ignoring request to move user {} to team {:?}.",
                &user_id, &team_to_move_to
            );
            return;
        }
    }

    eprintln!("Moving user {} to team A", &user_id);

    // update game state
    let new_game_state = prev_game_state.move_to_team(&team_to_move_to, &user_id);
    *write_games
        .get_mut(&game_id_clone)
        .expect(GAME_ID_NOT_IN_MAP) = new_game_state.clone();
    drop(write_games);

    // send moved teams event
    let moved_teams_event = match team_to_move_to {
        &TeamOption::TeamA => STCMsg::UserMovedToTeamA(user_id.to_string()),
        &TeamOption::TeamB => STCMsg::UserMovedToTeamB(user_id.to_string()),
    };
    send_ws_message::to_group(
        &game_id_clone,
        moved_teams_event,
        connections,
        games,
        game_codes,
    )
    .await;

    // send updated game state
    send_ws_message::game_state_to_group(
        &game_id_clone,
        &new_game_state,
        connections,
        games,
        game_codes,
    )
    .await;
}
