mod string_utils;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use string_utils::*;
use uuid::Uuid;
pub const NO_USER_ID: &str = "NO_USER_ID";
pub const NO_GAME_UD: &str = "NO_GAME_UD";

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum TichuCallStatus {
    Called,
    Achieved,
    Failed,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum TeamOption {
    TeamA,
    TeamB,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Team {
    pub id: String,
    pub team_name: String,
    pub user_ids: Vec<String>,
    pub score: i32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct TeamsState(pub Team, pub Team);

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum GameStage {
    Lobby,
    Teams(TeamsState),
    GrandTichu,
    Trade,
    Game,
    Scoreboard,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct PublicGameState {
    pub game_id: String,
    pub game_code: String,
    pub owner_id: String,
    pub stage: GameStage,
    pub participants: Vec<PublicUser>,
    pub current_user: PrivateUser,
}

/// Game state that only the server knows about.
/// This includes every user's cards, what is in the rest of the deck, etc.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct PrivateGameState {
    pub game_id: String,
    pub game_code: String,
    pub owner_id: String,
    pub stage: GameStage,
    pub participants: Vec<PrivateUser>,
    // small_tichus: Vec<TichuCallStatus>,
    // grand_tichus: Vec<TichuCallStatus>,
    // teams: [Team; 2],
    // active_player: String,
    // card_wished_for: Card,
    // deck: Vec<Card>,
    // discard: Vec<Card>,
    // in_play: Vec<Card>,
}

impl PrivateGameState {
    pub fn new(
        owner_id: String,
        owner_display_name: String,
        existing_game_codes: &HashMap<String, String>,
    ) -> PrivateGameState {
        let owner_user = PrivateUser {
            display_name: owner_display_name.clone(),
            user_id: owner_id.clone(),
            role: UserRole::Owner,
            tricks: vec![],
            hand: vec![],
        };
        let first_game_state = PrivateGameState {
            game_id: Uuid::new_v4().to_string(),
            game_code: get_new_game_code(existing_game_codes),
            stage: GameStage::Lobby,
            participants: vec![owner_user],
            owner_id: owner_id.clone(),
        };
        first_game_state
    }

    pub fn add_user(&self, user_id: String, display_name: String) -> PrivateGameState {
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

        let participant = PrivateUser {
            display_name,
            user_id: user_id.clone(),
            role: UserRole::Participant,
            tricks: vec![],
            hand: vec![],
        };
        let mut new_participants = self.participants.clone();
        new_participants.push(participant);

        // if 4 have joined, the new game stage should become Teams
        let new_stage = if current_participants == 3 {
            let team_a = Team {
                id: Uuid::new_v4().to_string(),
                score: 0,
                team_name: "Team A".into(),
                user_ids: vec![user_id, self.participants.get(0).unwrap().user_id.clone()],
            };

            let team_b = Team {
                id: Uuid::new_v4().to_string(),
                score: 0,
                team_name: "Team B".into(),
                user_ids: vec![
                    self.participants.get(1).unwrap().user_id.clone(),
                    self.participants.get(2).unwrap().user_id.clone(),
                ],
            };

            GameStage::Teams(TeamsState(team_a, team_b))
        } else {
            GameStage::Lobby
        };

        // clone old game state and update only what's necessary
        let mut new_game_state = self.clone();
        new_game_state.participants = new_participants;
        new_game_state.stage = new_stage;

        new_game_state
    }

    pub fn remove_user(&self, user_id: &str) -> PrivateGameState {
        let mut new_participants = self.participants.clone();

        // filter out removed user
        new_participants.retain(|user| user.user_id != user_id);

        // clone old game state and update only what's necessary
        let mut new_game_state = self.clone();
        new_game_state.participants = new_participants;
        new_game_state
    }

    pub fn reassign_owner(&self) -> PrivateGameState {
        // clone old game state and update only what's necessary
        let mut new_game_state = self.clone();
        let mut new_owner = new_game_state
            .participants
            .iter_mut()
            .find(|user| user.user_id != new_game_state.owner_id);
        let new_owner = new_owner.as_deref_mut();
        match new_owner {
            Some(new_owner) => {
                new_owner.role = UserRole::Owner;
                new_game_state.owner_id = new_owner.user_id.clone();
                new_game_state
            }
            None => new_game_state,
        }
    }

    /// Converts game state that only the server can see into state relevant for a specific user.
    pub fn to_public_game_state(&self, current_user_id: &str) -> Option<PublicGameState> {
        let mut public_participants: Vec<PublicUser> = Vec::with_capacity(4);
        let mut current_user = None;
        for private_participant in self.participants.iter() {
            let public_participant = PublicUser {
                display_name: private_participant.display_name.clone(),
                role: private_participant.role.clone(),
                tricks: private_participant.tricks.clone(),
                user_id: private_participant.user_id.clone(),
            };
            public_participants.push(public_participant);

            if private_participant.user_id == current_user_id {
                current_user = Some(private_participant.clone());
            }
        }

        if current_user.is_none() {
            eprintln!("Can't convert PrivateGameState to PublicGameState, because current user does not exist in list of participants");
            return None;
        }

        let public_game_state = PublicGameState {
            game_id: self.game_id.clone(),
            game_code: self.game_code.clone(),
            owner_id: self.owner_id.clone(),
            stage: self.stage.clone(),
            participants: public_participants,
            current_user: current_user.expect("Current user not found in participants"),
        };

        Some(public_game_state)
    }

    pub fn move_to_team(
        &self,
        team_to_move_to: &TeamOption,
        current_user_id: &str,
    ) -> PrivateGameState {
        let mut new_state = self.clone();
        match &mut new_state.stage {
            GameStage::Teams(teams) => {
                //if user is on the team they want to move to already, return
                let new_team = match team_to_move_to {
                    TeamOption::TeamA => &teams.0,
                    TeamOption::TeamB => &teams.1,
                };
                if new_team
                    .user_ids
                    .iter()
                    .find(|user_id| **user_id == current_user_id)
                    .is_some()
                {
                    return new_state;
                } else {
                    // remove user from team they were on before
                    let prev_team = match team_to_move_to {
                        TeamOption::TeamA => &mut teams.1,
                        TeamOption::TeamB => &mut teams.0,
                    };
                    prev_team
                        .user_ids
                        .retain(|user_id| user_id != current_user_id);

                    // add user to the new team
                    let new_team = match team_to_move_to {
                        TeamOption::TeamA => &mut teams.0,
                        TeamOption::TeamB => &mut teams.1,
                    };
                    new_team.user_ids.push(current_user_id.to_string());
                    new_state
                }
            }
            // game stage is not teams, can't move teams
            _ => new_state,
        }
    }

    pub fn rename_team(
        &self,
        team_to_rename: &TeamOption,
        current_user_id: &str,
        new_team_a_name: &str,
    ) -> PrivateGameState {
        let mut new_state = self.clone();
        match &mut new_state.stage {
            GameStage::Teams(teams) => {
                // user is on opposite team, so can't rename this team
                let opposite_team = match team_to_rename {
                    TeamOption::TeamA => &teams.1,
                    TeamOption::TeamB => &teams.0,
                };
                if opposite_team
                    .user_ids
                    .iter()
                    .find(|user_id| **user_id == current_user_id)
                    .is_some()
                {
                    return new_state;
                } else {
                    // rename intended team
                    let team_to_rename = match team_to_rename {
                        TeamOption::TeamA => &mut teams.0,
                        TeamOption::TeamB => &mut teams.1,
                    };
                    team_to_rename.team_name = new_team_a_name.to_string();
                    new_state
                }
            }
            // game stage is not teams, can't rename any team
            _ => new_state,
        }
    }
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

/// Contain's public information about the user:
/// Does NOT contain the user's hand.
/// This is important when sending data to other participants.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct PublicUser {
    pub user_id: String,
    pub role: UserRole,
    pub display_name: String,
    pub tricks: Vec<Vec<Card>>,
}

/// Contains private user data, including the user's hand, etc.
/// This data should be shared carefully to mitigate cheating.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct PrivateUser {
    pub user_id: String,
    pub role: UserRole,
    pub display_name: String,
    pub tricks: Vec<Vec<Card>>,
    pub hand: Vec<Card>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum User {
    PrivateUser(PrivateUser),
    PublicUser(PublicUser),
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
    /// Join a pre-existing game as the participant
    JoinGameWithGameCode(JoinGameWithGameCode),
    JoinRandomGame(JoinRandomGame),

    /// Create game as the owner
    CreateGame(CreateGame),

    /// User can only explicitly leave game when in the lobby
    LeaveGame,
    MoveToTeamA,
    MoveToTeamB,
    RenameTeamA(String),
    RenameTeamB(String),
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
    GameState(Option<PublicGameState>),

    /// The game owner has changed to be a different user.
    /// This can occur if the owner of the room leaves while still waiting in the lobby.
    OwnerReassigned(String),
    GameStageChanged(GameStage),
    TeamARenamed(String),
    TeamBRenamed(String),
    UserJoined(String),
    UserMovedToTeamA(String),
    UserMovedToTeamB(String),

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
