use crate::{
    CallGrandTichuRequest, Card, PublicGameStage, PublicGameState, SubmitTrade, TeamOption,
};
use serde::{Deserialize, Serialize};

/// All possible Server-to-Client Websocket Messages
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum STCMsg {
    UserIdAssigned(String),
    GameCreated {
        game_id: String,
        game_code: String,
    },

    /// Game state update
    /// Should only be None if the game completely ends and all users are removed
    GameState(Box<Option<PublicGameState>>),

    /// The game owner has changed to be a different user.
    /// This can occur if the owner of the room leaves while still waiting in the lobby.
    OwnerReassigned(String),
    GameStageChanged(PublicGameStage),
    TeamARenamed(String),
    TeamBRenamed(String),
    UserJoined(String),
    UserMovedToTeamA(String),
    UserMovedToTeamB(String),

    /// completely left game--not coming back.
    /// For now, this can only occur in the lobby.
    UserLeft(String),
    SmallTichuCalled(String),
    GrandTichuCalled(String, CallGrandTichuRequest),

    /// deal first 9 cards
    FirstCardsDealt,
    /// Deal last 5 cards.
    LastCardsDealt,

    /// Player with the Mah Jong leads.
    TradeSubmitted(String),

    /// after all submitted
    CardsPlayed,
    PlayerReceivedDragon,

    /// Temporary end game: show scores, etc.
    /// Users can restart if they want to play another game.
    GameEnded,

    /// Game state has been entirely cleaned up and no longer exists on the server.
    /// All users are completely ejected. There is no possibility of restarting without
    /// creating a new game.
    GameEndedFinal,

    Ping,
    Pong,
    Test(String),
    UnexpectedMessageReceived(String),

    /// Temporarily disconnected, but still in game.
    UserDisconnected(String),

    /// User previously disconnected, but now reconnected.
    UserReconnected(String),
}

/// All possible Client-to-Server Websocket Messages
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum CTSMsg {
    /// Join a pre-existing game as the participant
    JoinGameWithGameCode {
        user_id: String,
        display_name: String,
        game_code: String,
    },

    /// Create game as the owner
    CreateGame {
        user_id: String,
        display_name: String,
    },

    /// User can only explicitly leave game when in the lobby
    LeaveGame,
    MoveToTeam(TeamOption),

    RenameTeam {
        team_option: TeamOption,
        team_name: String,
    },

    /// Move from Teams stage to Grand Tichu stage
    StartGrandTichu,

    CallGrandTichu(CallGrandTichuRequest),

    CallSmallTichu,

    SubmitTrade(SubmitTrade),

    /// Submit cards to play in the Play stage
    PlayCards {
        cards: Vec<Card>,
        wished_for_card: Option<Card>,
    },

    Pass {
        user_id_to_give_dragon_to: Option<String>,
    },

    Ping,
    Pong,
    Test(String),
}
