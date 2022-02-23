use super::send_ws_message;
use crate::{Connections, GameCodes, Games};
use common::{PrivateGameStage, STCMsg};

const FUNCTION_NAME: &str = "cleanup_state_after_disconnect";

/// When a user disconnects, clean up their connection state and any game state they're associated with.
pub async fn cleanup_state_after_disconnect(
    user_id: &str,
    connections: &Connections,
    games: &Games,
    game_codes: &GameCodes,
) {
    eprintln!("{FUNCTION_NAME}: User disconnected: {}", user_id);

    let mut write_connections = connections.write().await;
    let mut write_games = games.write().await;
    let mut write_game_codes = game_codes.write().await;

    // extract all needed game state
    let game_id_clone = if let Some(connection) = &write_connections.get(user_id) {
        if let Some(game_id) = &connection.game_id {
            game_id.clone()
        } else {
            // user was not associated with any game (shouldn't happen), remove from Connections
            eprintln!("{FUNCTION_NAME}: Can't cleanup game state for user {}, because user was not associated with a game", user_id);
            write_connections.remove(user_id);
            return;
        }
    } else {
        // user_id was not found in Connections hash map, do nothing
        eprintln!("{FUNCTION_NAME}: Can't cleanup Connections state for user {}, because user was not found in the Connections HashMap", user_id);
        return;
    };
    let game_state_clone = match write_games.get(&game_id_clone) {
        Some(game_state) => game_state.clone(),
        None => {
            eprintln!("{FUNCTION_NAME}: Couldn't find associated game for game_id that was saved in Connection object for user {}", user_id);
            return;
        }
    };
    let game_code_clone = game_state_clone.game_code.clone();
    let participants_clone = game_state_clone.participants.clone();

    // check if any other participants are still connected
    let mut any_other_user_is_still_in_game = false;
    for participant in participants_clone.iter() {
        if participant.user_id != user_id {
            let participant_connection = match write_connections.get(&participant.user_id) {
                Some(participant_connection) => participant_connection,
                // participant not found in ConnectionsMap
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
            // if this is the lobby, remove from state
            eprintln!(
                "{FUNCTION_NAME}: Removing user {} from lobby on disconnect",
                user_id
            );
            let mut owner_reassigned = false;
            write_connections.remove(user_id);
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

            let game = match write_games.get_mut(&game_id_clone) {
                Some(game) => game,
                None => {
                    eprintln!("{FUNCTION_NAME}: Couldn't remove user_id {} from game state because game was not found via the game_id", user_id);
                    return;
                }
            };

            // update game state without the participant there
            *game = new_game_state.clone();

            drop(write_connections);
            drop(write_games);
            drop(write_game_codes);

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

            // send updated game state
            send_ws_message::game_state_to_group(
                &game_id_clone,
                &new_game_state,
                connections,
                games,
                game_codes,
            )
            .await;
        } else {
            // no other users left in game and user is not in lobby: only mark this user as disconnected
            eprintln!("{FUNCTION_NAME} Marking user {} as not connected", user_id);

            if let Some(connection_data) = write_connections.get_mut(user_id) {
                connection_data.connected = false;
            } else {
                eprintln!("User not {} not found in connection map", user_id);
            }

            drop(write_connections);
            drop(write_games);
            drop(write_game_codes);

            // notify remaining participants that user was disconnected
            send_ws_message::to_group(
                &game_id_clone,
                STCMsg::UserDisconnected(user_id.to_string()),
                connections,
                games,
                game_codes,
            )
            .await;

            // send old game state (no change occurred)
            send_ws_message::game_state_to_group(
                &game_id_clone,
                &game_state_clone,
                connections,
                games,
                game_codes,
            )
            .await;
        }
    } else {
        // no users left in game: remove all users from Connections and delete game from Games
        eprintln!(
            "{FUNCTION_NAME} Removing all users and game from state for game {}",
            game_id_clone
        );
        for participant in participants_clone.iter() {
            write_connections.remove(&participant.user_id);
        }
        write_games.remove(&game_id_clone);
        write_game_codes.remove(&game_code_clone);
    }
}
