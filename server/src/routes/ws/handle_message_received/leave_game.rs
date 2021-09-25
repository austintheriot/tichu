use crate::{routes::ws::send_game_state_to_all_participants, ws::send_ws_message_to_user};
use common::{GameStage, STCMsg};

use crate::{
    errors::{GAME_ID_NOT_IN_MAP, USER_ID_NOT_IN_MAP},
    routes::ws::send_ws_message_to_all_participants,
    Connections, GameCodes, Games,
};

pub async fn leave_game(
    user_id: &str,
    connections: &Connections,
    games: &Games,
    game_codes: &GameCodes,
) {
    let mut write_connections = connections.write().await;
    let mut write_games = games.write().await;
    let mut write_game_codes = game_codes.write().await;

    // extract game_id
    // if user is not in a game, no action needed
    let game_id_clone = match &write_connections
        .get(user_id)
        .expect(USER_ID_NOT_IN_MAP)
        .game_id
    {
        Some(game_id) => game_id.clone(),
        None => return,
    };

    // extract all needed game state
    let game_state_clone = write_games
        .get(&game_id_clone)
        .expect(GAME_ID_NOT_IN_MAP)
        .clone();
    let game_code_clone = game_state_clone.game_code.clone();
    let participants_clone = game_state_clone.participants.clone();

    // check if any other participants are still in room
    let mut any_other_user_is_still_in_game = false;
    for participant in participants_clone.iter() {
        if participant.user_id != user_id {
            // should always connected in lobby (since users
            // are just removed from the lobby when they disconnect)
            // but it doesn't hurt to check
            let participant_connection = write_connections
                .get(&participant.user_id)
                .expect(USER_ID_NOT_IN_MAP);
            if participant_connection.connected {
                any_other_user_is_still_in_game = true;
            }
        }
    }

    // other users still in game:
    if any_other_user_is_still_in_game {
        if let GameStage::Lobby = game_state_clone.stage {
            // if this is the lobby, remove user from the lobby, but keep connection open
            eprintln!(
                "Removing user {} from lobby on leave game event, but keeping connection open",
                user_id
            );
            let mut owner_reassigned = false;

            // update game state by removing user and reassigning owner if needed
            let new_game_state = if game_state_clone.owner_id == *user_id {
                // if owner leaves in lobby, assign ownership to next participant
                eprintln!("Reassigning owner role to a different user");
                owner_reassigned = true;
                game_state_clone.remove_user(&user_id).reassign_owner()
            } else {
                // if not the owner, just remove from state
                game_state_clone.remove_user(&user_id)
            };
            *write_games
                .get_mut(&game_id_clone)
                .expect(GAME_ID_NOT_IN_MAP) = new_game_state.clone();

            // disassociate user_id with game
            write_connections
                .get_mut(user_id)
                .expect(USER_ID_NOT_IN_MAP)
                .game_id = None;

            drop(write_connections);
            drop(write_games);
            drop(write_game_codes);

            // notify remaining participants that user left
            send_ws_message_to_all_participants(
                &game_id_clone,
                STCMsg::UserLeft(user_id.to_string()),
                connections,
                games,
                game_codes,
            )
            .await;

            // notify remaining participants that new owner was chosen
            if owner_reassigned {
                send_ws_message_to_all_participants(
                    &game_id_clone,
                    STCMsg::OwnerReassigned(new_game_state.owner_id.clone()),
                    connections,
                    games,
                    game_codes,
                )
                .await;
            }

            // send updated game state to other participants
            send_game_state_to_all_participants(
                &game_id_clone,
                &new_game_state,
                connections,
                games,
                game_codes,
            )
            .await;

            // send a None game state to current user
            send_ws_message_to_user(&user_id, STCMsg::GameState(None), connections).await;
        } else {
            // user not in lobby: can't leave
            eprintln!(
                "User {} can't leave game since user is not in lobby",
                user_id
            );
            return;
        }
    } else {
        // no other users left in game: delete game but keep user connection
        eprintln!(
            "Removing user {} from game state and deleting game {}",
            user_id, game_id_clone
        );

        write_games.remove(&game_id_clone);
        write_game_codes.remove(&game_code_clone);

        // disassociate user with game_id
        write_connections
            .get_mut(user_id)
            .expect(USER_ID_NOT_IN_MAP)
            .game_id = None;

        drop(write_connections);
        drop(write_games);
        drop(write_game_codes);

        // send a None game state to current user
        send_ws_message_to_user(&user_id, STCMsg::GameState(None), connections).await;
    }
}
