mod call_grand_tichu;
mod create_game;
mod join_game_with_game_code;
mod leave_game;
mod move_to_team;
mod ping;
mod pong;
mod rename_team;
mod start_grand_tichu;
mod test;

use call_grand_tichu::call_grand_tichu;
use create_game::create_game;
use join_game_with_game_code::join_game_with_game_code;
use leave_game::leave_game;
use move_to_team::move_to_team;
use ping::ping;
use pong::pong;
use rename_team::rename_team;
use start_grand_tichu::start_grand_tichu;
use test::test;

use super::send_ws_message;
use crate::{Connections, GameCodes, Games};
use common::{CTSMsg, RenameTeam, STCMsg};
use warp::ws::Message;

pub async fn handle_message_received(
    user_id: String,
    msg: Message,
    connections: &Connections,
    games: &Games,
    game_codes: &GameCodes,
) {
    if !msg.is_binary() {
        eprintln!("Text websocket message received: {:?}", &msg);
        return;
    }

    let msg: CTSMsg = bincode::deserialize(&msg.as_bytes()).expect("Could not serialize message");

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
        CTSMsg::CreateGame(create_game_data) => {
            create_game(create_game_data, &connections, &games, &game_codes).await;
        }
        CTSMsg::JoinGameWithGameCode(join_game_with_game_code_data) => {
            join_game_with_game_code(
                join_game_with_game_code_data,
                &connections,
                &games,
                &game_codes,
            )
            .await;
        }
        CTSMsg::LeaveGame => {
            leave_game(&user_id, &connections, &games, &game_codes).await;
        }
        CTSMsg::MoveToTeam(team_option) => {
            move_to_team(&team_option, &user_id, &connections, &games, &game_codes).await;
        }
        CTSMsg::RenameTeam(rename_team_data) => {
            let RenameTeam {
                team_name: new_team_name,
                team_option,
            } = rename_team_data;
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
        any_other_message => {
            eprintln!("Unexpected message received: {:?}", any_other_message);

            // let user know something weird was received
            send_ws_message::to_user(
                &user_id,
                STCMsg::UnexpectedMessageReceived(format!("{:#?}", &any_other_message)),
                &connections,
            )
            .await;
        }
    }
}
