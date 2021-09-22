mod utils;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use uuid::Uuid;

use crate::utils::get_new_game_code;

pub const NO_USER_ID: &str = "NO_USER_ID";
pub const NO_GAME_UD: &str = "NO_GAME_UD";

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum TichuCallStatus {
    Called,
    Achieved,
    Failed,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum GameStage {
    Lobby,
    Teams,
    GrandTichu,
    Trade,
    Game,
    Endgame,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct GameState {
    pub game_id: String,
    pub game_code: String,
    pub owner_id: String,
    pub stage: GameStage,
    pub participants: Vec<User>,
    // small_tichus: Vec<TichuCallStatus>,
    // grand_tichus: Vec<TichuCallStatus>,
    // teams: [Team; 2],
    // owner_id: String,
    // active_player: String,
    // card_wished_for: Card,
    // deck: Vec<Card>,
    // discard: Vec<Card>,
    // in_play: Vec<Card>,
}

impl GameState {
    pub fn new(
        owner_id: String,
        owner_display_name: String,
        existing_game_codes: &HashMap<String, String>,
    ) -> GameState {
        let owner_user = User {
            display_name: owner_display_name.clone(),
            user_id: owner_id.clone(),
            role: UserRole::Owner,
            tricks: vec![],
            hand: vec![],
        };
        let first_game_state = GameState {
            game_id: Uuid::new_v4().to_string(),
            game_code: get_new_game_code(existing_game_codes),
            stage: GameStage::Lobby,
            participants: vec![owner_user],
            owner_id: owner_id.clone(),
        };
        first_game_state
    }

    pub fn add_user(&mut self, user_id: String, display_name: String) -> GameState {
        let current_participants = self.participants.len();
        let game_has_max_participants = current_participants == 4;
        let is_lobby = match self.stage {
            GameStage::Lobby => true,
            _ => false,
        };

        // don't add any more than 4 users at a time
        if !is_lobby || game_has_max_participants {
            return self.clone();
        }

        // if 4 have joined, the new game stage should become Teams
        let new_stage = if current_participants == 3 {
            GameStage::Teams
        } else {
            GameStage::Lobby
        };
        let participant = User {
            display_name,
            user_id,
            role: UserRole::Participant,
            tricks: vec![],
            hand: vec![],
        };
        let mut new_participants = self.participants.clone();
        new_participants.push(participant);

        // clone old game state and update only what's necessary
        let mut new_game_state = self.clone();
        new_game_state.participants = new_participants;
        new_game_state.stage = new_stage;

        new_game_state
    }

    pub fn remove_user(&self, user_id: &str) -> GameState {
        let mut new_participants = self.participants.clone();

        // filter out removed user
        new_participants.retain(|user| user.user_id != user_id);

        // clone old game state and update only what's necessary
        let mut new_game_state = self.clone();
        new_game_state.participants = new_participants;
        new_game_state
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Team {
    id: String,
    users: Vec<String>,
    score: i32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum CardValue {
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    _10,
    J,
    Q,
    K,
    A,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Card {
    Sword(CardValue),
    Jade(CardValue),
    Pagoda(CardValue),
    Star(CardValue),
    Dragon,
    Phoenix,
    MahJong,
    Dog,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Trade {
    from: String,
    to: String,
    card: Card,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum UserRole {
    Owner,
    Participant,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct User {
    pub user_id: String,
    pub role: UserRole,
    pub display_name: String,
    pub hand: Vec<Card>,
    pub tricks: Vec<Vec<Card>>,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct CreateGame {
    pub user_id: String,
    pub display_name: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct JoinGameWithGameCode {
    pub user_id: String,
    pub display_name: String,
    pub game_code: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct JoinRandomGame {
    pub user_id: String,
    pub display_name: String,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct ChooseTeamMessage {
    pub team_id: String,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct RenameTeam {
    pub team_name: String,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct SubmitTrade {
    pub trade_to: String,
    pub card: Card,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct PlayCard {
    pub cards: Vec<Card>,
    pub wished_for: Option<Card>,
    pub give_dragon_to: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct GiveDragon {
    pub user_id: String,
}

/// Client to Server Websocket Messages
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum CTSMsg {
    JoinGameWithGameCode(JoinGameWithGameCode),
    JoinRandomGame(JoinRandomGame),
    CreateGame(CreateGame),
    RenameTeam(RenameTeam),
    StartGame,
    SubmitTrade(SubmitTrade),
    PlayCards(PlayCard),
    GiveDragon(GiveDragon),

    Ping,
    Pong,
    Test(String),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct GameCreated {
    pub game_id: String,
    pub game_code: String,
}

/// Server to Client Websocket Messages
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum STCMsg {
    UserIdAssigned(String),
    GameCreated(GameCreated),

    /// Game state update
    /// Should only be None if the game completely ends and all users are removed
    GameState(Option<GameState>),
    GameStageChanged(GameStage),
    TeamRenamed,
    UserJoined(String),

    /// completely left game--not coming back.
    /// For now, this can only occur in the lobby.
    UserLeft(String),
    SmallTichuCalled,
    GrandTichuCalled,
    StartGame,

    /// deal first 9 cards
    DealFinalCards,

    /// Deal last 5 cards.
    /// Player with the Mah Jong leads.
    CardsTraded,

    /// after all submitted
    CardsPlayed,
    DragonWasWon,
    PlayerReceivedDragon,

    /// show scores, etc.
    EndGame,

    /// game state has been entirely cleaned up and no longer exists on the server
    GameOver,

    Ping,
    Pong,
    Test(String),
    UnexpectedMessageReceived(String),

    /// Temporarily disconnected, but still in game.
    UserDisconnected(String),

    /// User previously disconnected, but now reconnected.
    UserReconnected(String),
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
