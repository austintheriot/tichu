use crate::routes::ws::send_ws_message;
use crate::ConnectionData;
use crate::{Connections, GameCodes, Games};
use common::{PrivateGameStage, STCMsg};

const FUNCTION_NAME: &str = "leave_game";

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
    let game_id_clone = if let Some(ConnectionData {
        game_id: Some(game_id),
        ..
    }) = &write_connections.get(user_id)
    {
        game_id.clone()
    } else {
        eprintln!("{FUNCTION_NAME}: User {user_id} can't leave game because their user_id could not be found in the Connections HashMap");
        return;
    };

    // extract all needed game state
    let game_state_clone = if let Some(game_state) = write_games.get(&game_id_clone) {
        game_state.clone()
    } else {
        eprintln!("{FUNCTION_NAME}: User {user_id} can't leave game because game_state could not be found in Games HashMap");
        return;
    };

    let game_code_clone = game_state_clone.game_code.clone();
    let participants_clone = game_state_clone.participants.clone();

    // check if any other participants are still in room
    let mut any_other_user_is_still_in_game = false;
    for participant in participants_clone.iter() {
        if participant.user_id != user_id {
            // should always be connected in lobby (since users
            // are just removed from the lobby when they disconnect)
            // but it doesn't hurt to check
            let participant_connection = match write_connections.get(&participant.user_id) {
                Some(participant_connection) => participant_connection,
                None => continue,
            };
            if participant_connection.connected {
                any_other_user_is_still_in_game = true;
            }
        }
    }

    // other users still in game:
    if any_other_user_is_still_in_game {
        if let PrivateGameStage::Lobby = game_state_clone.stage {
            // if this is the lobby, remove user from the lobby, but keep connection open
            eprintln!(
                "{FUNCTION_NAME}: Removing user {} from game, but keeping user's connection open",
                user_id
            );
            let mut owner_reassigned = false;

            // update game state by removing user and reassigning owner if needed
            let new_game_state = if game_state_clone.owner_id == *user_id {
                // if owner leaves in lobby, assign ownership to next participant
                eprintln!("{FUNCTION_NAME}: Reassigning owner role to a different user");
                owner_reassigned = true;
                match game_state_clone.remove_user(user_id) {
                    Ok(updated_game_state) => match updated_game_state.reassign_owner() {
                        Ok(updated_game_state) => updated_game_state,
                        Err(err) => return eprintln!("{}", err),
                    },
                    Err(err) => return eprintln!("{}", err),
                }
            } else {
                // if not the owner, just remove from state
                match game_state_clone.remove_user(user_id) {
                    Ok(updated_game_state) => updated_game_state,
                    Err(err) => return eprintln!("{}", err),
                }
            };

            // update game state
            if let Some(game_state) = write_games.get_mut(&game_id_clone) {
                *game_state = new_game_state.clone();
            }

            // disassociate user_id with game
            if let Some(connection_data) = write_connections.get_mut(user_id) {
                connection_data.game_id = None;
            }

            drop(write_connections);
            drop(write_games);
            drop(write_game_codes);

            eprintln!("{FUNCTION_NAME}: User {} successfully left game. Other users are still in the game so persisting game state", user_id);

            // notify remaining participants that user left
            send_ws_message::to_group(
                &game_id_clone,
                STCMsg::UserLeft(user_id.to_string()),
                connections,
                games,
                game_codes,
            )
            .await;

            // notify remaining participants that new owner was chosen
            if owner_reassigned {
                send_ws_message::to_group(
                    &game_id_clone,
                    STCMsg::OwnerReassigned(new_game_state.owner_id.clone()),
                    connections,
                    games,
                    game_codes,
                )
                .await;
            }

            // send updated game state to other participants
            send_ws_message::game_state_to_group(
                &game_id_clone,
                &new_game_state,
                connections,
                games,
                game_codes,
            )
            .await;

            // send a None game state to current user
            send_ws_message::to_user(user_id, STCMsg::GameState(Box::new(None)), connections).await;
        } else {
            // user not in lobby: can't leave
            eprintln!(
                "{FUNCTION_NAME}: User {} can't leave game since user is not in Lobby game stage",
                user_id
            );
        }
    } else {
        // no other users left in game: delete game but keep user connection
        eprintln!(
            "{FUNCTION_NAME}: Removing user {} from game state. No users left in game, so deleting game {}",
            user_id, game_id_clone
        );

        write_games.remove(&game_id_clone);
        write_game_codes.remove(&game_code_clone);

        // disassociate user with game_id
        if let Some(connection_data) = write_connections.get_mut(user_id) {
            connection_data.game_id = None;
        }

        drop(write_connections);
        drop(write_games);
        drop(write_game_codes);

        // send a None game state to current user
        send_ws_message::to_user(user_id, STCMsg::GameState(Box::new(None)), connections).await;
    }
}
