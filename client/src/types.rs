use serde::{Deserialize, Serialize};

/// Internal Tichu-client message for alerting that it's time to send a websocket message
///
/// This type reflects the common::CTSMsg, except with all data values tripped,
/// since the data values are formulated in the send_ws_message message
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum CTSMsgInternal {
    JoinGameWithGameCode,
    JoinRandomGame,
    MoveToTeamA,
    MoveToTeamB,
    RenameTeamA,
    RenameTeamB,
    CreateGame,
    LeaveGame,
    RenameTeam,
    StartGame,
    SubmitTrade,
    PlayCards,
    GiveDragon,

    Ping,
    Pong,
    Test,
}
