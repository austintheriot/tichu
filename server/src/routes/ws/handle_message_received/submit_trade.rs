use crate::{routes::ws::send_ws_message, Connections, GameCodes, Games};
use common::{PrivateGameStage, STCMsg, SubmitTrade};

const FUNCTION_NAME: &str = "submit_trade";

pub async fn submit_trade(
    trade_array: SubmitTrade,
    user_id: &str,
    connections: &Connections,
    games: &Games,
    game_codes: &GameCodes,
) {
    let read_connections = connections.read().await;
    let mut write_games = games.write().await;

    let user = match read_connections.get(user_id) {
        Some(user) => user,
        None => {
            eprintln!("{FUNCTION_NAME}: User {} can't submit trade, because their user_id could not be found in the Connections HashMap", user_id);
            return;
        }
    };

    let game_id = match user.game_id.clone() {
        Some(game_id) => game_id,
        None => {
            eprintln!("{FUNCTION_NAME}: User {} can't submit trade, because they are not associated with a game_id", user_id);
            return;
        }
    };

    let game_state = match write_games.get_mut(&game_id) {
        Some(game_state) => game_state,
        None => {
            eprintln!("{FUNCTION_NAME}: User {} can't submit trade, because the game they are associated with could not be found in the Games HashMap", user_id);
            return;
        }
    };

    // must be Trade stage
    if let PrivateGameStage::Trade(trade_stage) = &game_state.stage {
        let i = game_state
            .participants
            .iter()
            .position(|user| *user.user_id == *user_id);

        let user = if let Some(i) = i {
            &mut game_state.participants[i]
        } else {
            eprintln!("{FUNCTION_NAME}: couldn't accept traded submitted by user {user_id} because user could not be found in participants");
            return;
        };

        for trade in &trade_array {
            // User must actually have those cards in their hand
            if !user.hand.contains(&trade.card) {
                eprintln!("{FUNCTION_NAME}: Couldn't accept traded submitted by user {} because user does {:?}, which they are trying to trade", user_id, trade.card);
                return;
            }

            // Trade must not be to self
            if trade.to_user_id == user_id {
                eprintln!("{FUNCTION_NAME}: Couldn't accept traded submitted by user {} because user is trying to trade to self", user_id);
                return;
            }

            // Trade must be to a valid participant who is on a team
            let mut recipient_found_in_teams = false;
            for team in &trade_stage.teams {
                for id in team.user_ids.iter() {
                    if *id == *trade.to_user_id {
                        recipient_found_in_teams = true;
                    }
                }
            }
            if !recipient_found_in_teams {
                eprintln!("{FUNCTION_NAME}: Couldn't accept traded submitted by user {} because the person the user is trying to trade to was not found in the teams", user_id);
                return;
            }
        }
    } else {
        eprintln!(
            "Couldn't accept traded submitted by user {} because Game Stage is not Trade",
            user_id
        );
        return;
    }

    // update game state
    let new_game_state = match game_state.submit_trade(user_id, &trade_array) {
        Ok(updated_game_state) => updated_game_state,
        Err(err) => return eprintln!("{}", err),
    };
    *game_state = new_game_state.clone();

    drop(write_games);

    eprintln!(
        "{FUNCTION_NAME}: User {} successfully submitted trade",
        user_id
    );

    // send Trade submitted
    send_ws_message::to_group(
        &game_id,
        STCMsg::TradeSubmitted(user_id.to_string()),
        connections,
        games,
        game_codes,
    )
    .await;

    // if game stage changed to Play, send GameStageChanged event
    if let PrivateGameStage::Play(_) = &new_game_state.stage {
        eprintln!(
            "{FUNCTION_NAME}: Final trade submitted. Game state moved to Play for game {game_id}"
        );
        send_ws_message::to_group(
            &game_id,
            STCMsg::GameStageChanged(new_game_state.stage.clone().into()),
            connections,
            games,
            game_codes,
        )
        .await;
    }

    // send updated game state
    send_ws_message::game_state_to_group(&game_id, &new_game_state, connections, games, game_codes)
        .await;
}
