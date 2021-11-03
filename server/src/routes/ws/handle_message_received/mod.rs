mod call_grand_tichu;
mod call_small_tichu;
mod create_game;
mod join_game_with_game_code;
mod leave_game;
mod move_to_team;
mod ping;
mod play_cards;
mod pong;
mod rename_team;
mod start_grand_tichu;
mod submit_trade;
mod test;

use call_grand_tichu::call_grand_tichu;
use call_small_tichu::call_small_tichu;
use create_game::create_game;
use join_game_with_game_code::join_game_with_game_code;
use leave_game::leave_game;
use move_to_team::move_to_team;
use ping::ping;
use play_cards::play_cards;
use pong::pong;
use rename_team::rename_team;
use start_grand_tichu::start_grand_tichu;
use submit_trade::submit_trade;
use test::test;

use super::send_ws_message;
use crate::{Connections, GameCodes, Games};
use common::CTSMsg;
use warp::ws::Message;

pub async fn handle_message_received(
    user_id: String,
    msg: Message,
    connections: Connections,
    games: Games,
    game_codes: GameCodes,
) {
    if !msg.is_binary() {
        eprintln!("Text websocket message received: {:?}", &msg);
        return;
    }

    let msg: CTSMsg = bincode::deserialize(msg.as_bytes()).expect("Could not serialize message");

    match msg {
        CTSMsg::Test(_) => {
            test(&user_id, &connections).await;
        }
        CTSMsg::Ping => {
            ping(&user_id, &connections).await;
        }
        CTSMsg::Pong => {
            pong(&user_id, &connections).await;
        }
        CTSMsg::CreateGame { .. } => {
            create_game(msg, connections, &games, &game_codes).await;
        }
        CTSMsg::JoinGameWithGameCode { .. } => {
            join_game_with_game_code(msg, connections, games, game_codes).await;
        }
        CTSMsg::LeaveGame => {
            leave_game(&user_id, &connections, &games, &game_codes).await;
        }
        CTSMsg::MoveToTeam(team_option) => {
            move_to_team(&team_option, &user_id, &connections, &games, &game_codes).await;
        }
        CTSMsg::RenameTeam {
            team_name: new_team_name,
            team_option,
        } => {
            rename_team(
                &team_option,
                new_team_name,
                &user_id,
                &connections,
                &games,
                &game_codes,
            )
            .await;
        }
        CTSMsg::StartGrandTichu => {
            start_grand_tichu(&user_id, &connections, &games, &game_codes).await;
        }
        CTSMsg::CallGrandTichu(call_grand_tichu_request) => {
            call_grand_tichu(
                &call_grand_tichu_request,
                &user_id,
                &connections,
                &games,
                &game_codes,
            )
            .await;
        }
        CTSMsg::CallSmallTichu => {
            call_small_tichu(&user_id, &connections, &games, &game_codes).await;
        }
        CTSMsg::SubmitTrade(trade_array) => {
            submit_trade(trade_array, &user_id, &connections, &games, &game_codes).await;
        }
        CTSMsg::PlayCards {
            cards,
            give_dragon_to,
            wished_for,
        } => {
            play_cards(
                &user_id,
                cards,
                give_dragon_to,
                wished_for,
                &connections,
                &games,
                &game_codes,
            )
            .await;
        }
    }
}
