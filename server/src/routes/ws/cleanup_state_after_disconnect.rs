use super::send_ws_message;
use common::{PrivateGameStage, STCMsg};

use crate::{
    errors::{GAME_ID_NOT_IN_MAP, USER_ID_NOT_IN_MAP},
    Connections, GameCodes, Games,
};

/// When a user disconnects, clean up their connection state and any game state their associated with.
pub async fn cleanup_state_after_disconnect(
    user_id: &str,
    connections: &Connections,
    games: &Games,
    game_codes: &GameCodes,
) {
    eprintln!("User disconnected: {}", user_id);

    // extract game_id
    let mut write_connections = connections.write().await;
    let mut write_games = games.write().await;
    let mut write_game_codes = game_codes.write().await;
    let game_id_clone = &write_connections
        .get(user_id)
        .expect(USER_ID_NOT_IN_MAP)
        .game_id
        .clone();

    match game_id_clone {
        // user is associated with a game_id (should always be the case)
        Some(game_id) => {
            // extract all needed game state
            let game_state_clone = write_games.get(game_id).expect(GAME_ID_NOT_IN_MAP).clone();
            let game_code_clone = game_state_clone.game_code.clone();
            let participants_clone = game_state_clone.participants.clone();

            // check if any other participants are still connected
            let mut any_other_user_is_still_in_game = false;
            for participant in participants_clone.iter() {
                if participant.user_id != user_id {
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
                if let PrivateGameStage::Lobby = game_state_clone.stage {
                    // if this is the lobby, remove from state
                    eprintln!("Removing user {} from lobby on disconnect", user_id);
                    let mut owner_reassigned = false;
                    write_connections.remove(user_id);
                    let new_game_state = if game_state_clone.owner_id == *user_id {
                        // if owner leaves in lobby, assign ownership to next participant
                        eprintln!("Reassigning owner role to a different user");
                        owner_reassigned = true;
                        game_state_clone.remove_user(user_id).reassign_owner()
                    } else {
                        // if not the owner, just remove from state
                        game_state_clone.remove_user(user_id)
                    };

                    *write_games.get_mut(game_id).expect(GAME_ID_NOT_IN_MAP) =
                        new_game_state.clone();

                    drop(write_connections);
                    drop(write_games);
                    drop(write_game_codes);

                    // notify remaining participants that user left
                    send_ws_message::to_group(
                        game_id,
                        STCMsg::UserLeft(user_id.to_string()),
                        connections,
                        games,
                        game_codes,
                    )
                    .await;

                    // notify remaining participants that new owner was chosen
                    if owner_reassigned {
                        send_ws_message::to_group(
                            game_id,
                            STCMsg::OwnerReassigned(new_game_state.owner_id.clone()),
                            connections,
                            games,
                            game_codes,
                        )
                        .await;
                    }

                    // send updated game state
                    send_ws_message::game_state_to_group(
                        game_id,
                        &new_game_state,
                        connections,
                        games,
                        game_codes,
                    )
                    .await;
                } else {
                    // no other users left in game and user is not in lobby: only mark this user as disconnected
                    eprintln!("Marking user {} as not connected", user_id);
                    write_connections
                        .get_mut(user_id)
                        .expect(USER_ID_NOT_IN_MAP)
                        .connected = false;

                    drop(write_connections);
                    drop(write_games);
                    drop(write_game_codes);

                    // notify remaining participants that user was disconnected
                    send_ws_message::to_group(
                        game_id,
                        STCMsg::UserDisconnected(user_id.to_string()),
                        connections,
                        games,
                        game_codes,
                    )
                    .await;

                    // send old game state (no change occurred)
                    send_ws_message::game_state_to_group(
                        game_id,
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
                    "Removing all users and game from state for game {}",
                    game_id
                );
                for participant in participants_clone.iter() {
                    write_connections.remove(&participant.user_id);
                }
                write_games.remove(game_id);
                write_game_codes.remove(&game_code_clone);
            }
        }
        None => {
            // user was not associated with any game (shouldn't happen), remove from Connections
            write_connections.remove(user_id);
        }
    }
}
