use common::{PrivateGameStage, STCMsg, TeamOption};

use crate::{
    errors::{GAME_ID_NOT_IN_MAP, USER_ID_NOT_IN_MAP},
    routes::ws::send_ws_message,
    Connections, GameCodes, Games,
};

const FUNCTION_NAME: &str = "rename_team";

pub async fn rename_team(
    team_to_rename: &TeamOption,
    new_team_name: String,
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
                "{FUNCTION_NAME}: User {} is not associated with a game. Ignoring request to rename {:?}",
                &user_id, &team_to_rename,
            );
            return;
        }
        Some(game_id_clone) => game_id_clone,
    };
    let prev_game_state = write_games.get(&game_id_clone).expect(GAME_ID_NOT_IN_MAP);
    match &prev_game_state.stage {
        PrivateGameStage::Teams(teams_state) => {
            // if user is not on team they want to rename, ignore
            let opposite_team = match team_to_rename {
                TeamOption::TeamA => &teams_state[1],
                TeamOption::TeamB => &teams_state[0],
            };
            if opposite_team.user_ids.contains(&user_id.into()) {
                eprintln!(
                    "{FUNCTION_NAME}: User {} is not on the team they want to rename ({:?}). Ignoring request to rename team",
                    &user_id,
                    &team_to_rename,
                );
                return;
            }
        }
        // current stage is not Teams, do nothing
        _ => {
            eprintln!(
                "{FUNCTION_NAME}: Current stage is not Teams. Ignoring request from user {} to rename team {:?}",
                &user_id, &team_to_rename,
            );
            return;
        }
    }

    // update game state
    let new_game_state = match prev_game_state.rename_team(team_to_rename, user_id, &new_team_name)
    {
        Ok(updated_game_state) => updated_game_state,
        Err(err) => return eprintln!("{}", err),
    };
    *write_games
        .get_mut(&game_id_clone)
        .expect(GAME_ID_NOT_IN_MAP) = new_game_state.clone();
    drop(write_games);

    eprintln!(
        "{FUNCTION_NAME}: User {} successfully renamed team {:?} to {}",
        &user_id, &team_to_rename, &new_team_name
    );

    let team_renamed_event = match team_to_rename {
        TeamOption::TeamA => STCMsg::TeamARenamed(new_team_name),
        TeamOption::TeamB => STCMsg::TeamBRenamed(new_team_name),
    };
    // send team rename event
    send_ws_message::to_group(
        &game_id_clone,
        team_renamed_event,
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
