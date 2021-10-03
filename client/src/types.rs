use common::{CallGrandTichuRequest, TeamOption};
use serde::{Deserialize, Serialize};

/// Internal Tichu-client message for alerting that it's time to send a websocket message
///
/// This type reflects the common::CTSMsg, except with all data values tripped,
/// since the data values are formulated in the send_ws_message message
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum CTSMsgInternal {
    JoinGameWithGameCode,
    JoinRandomGame,
    MoveToTeam(TeamOption),
    RenameTeam(TeamOption),
    CreateGame,
    LeaveGame,
    StartGrandTichu,
    SubmitTrade,
    PlayCards,
    GiveDragon,
    CallGrandTichu(CallGrandTichuRequest),
    CallSmallTichu,

    Ping,
    Pong,
    Test,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum TradeToPerson {
    Opponent1,
    Teammate,
    Opponent2,
}
