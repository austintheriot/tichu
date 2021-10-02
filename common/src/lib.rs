#![feature(format_args_capture)]

extern crate rand;
use crate::rand::SeedableRng;
use rand::prelude::SliceRandom;
use rand::rngs::SmallRng;

mod string_utils;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

pub use string_utils::*;
use uuid::Uuid;
pub const NO_USER_ID: &str = "NO_USER_ID";
pub const NO_GAME_UD: &str = "NO_GAME_UD";

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum TichuCallStatus {
    /// User has not called one way or the other yet
    Undecided,

    /// User has called some form of Tichu
    Called,

    /// User has declined to call Tichu
    Declined,

    /// User has called Tichu and has successfully achieved it
    Achieved,

    /// User has called Tichu but failed to achieve it
    Failed,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct UserIdWithTichuCallStatus {
    pub user_id: String,
    pub tichu_call_status: TichuCallStatus,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum TeamOption {
    TeamA,
    TeamB,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct MutableTeam {
    pub id: String,
    pub team_name: String,
    pub user_ids: Vec<String>,
    pub score: i32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct ImmutableTeam {
    pub id: String,
    pub team_name: String,
    pub user_ids: [String; 2],
    pub score: i32,
}

impl TryFrom<MutableTeam> for ImmutableTeam {
    type Error = String;
    fn try_from(item: MutableTeam) -> Result<Self, String> {
        let user_id1 = item.user_ids.get(0);
        let user_id2 = item.user_ids.get(1);
        if user_id1.is_none() || user_id2.is_none() {
            return Err("Could not convert MutableTeam to ImmutableTeam, since there were not enough user_ids".to_string());
        }

        Ok(ImmutableTeam {
            id: item.id,
            score: item.score,
            team_name: item.team_name,
            user_ids: [user_id1.unwrap().to_string(), user_id2.unwrap().to_string()],
        })
    }
}

pub type MutableTeams = [MutableTeam; 2];

pub type ImmutableTeams = [ImmutableTeam; 2];

/// Client state: does NOT include sensitive information, such as the Deck
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct PublicGrandTichu {
    pub small_tichus: [UserIdWithTichuCallStatus; 4],
    pub grand_tichus: [UserIdWithTichuCallStatus; 4],
    pub teams: ImmutableTeams,
}

/// Server state: includes sensitive information, such as the Deck
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct PrivateGrandTichu {
    pub small_tichus: [UserIdWithTichuCallStatus; 4],
    pub grand_tichus: [UserIdWithTichuCallStatus; 4],
    pub teams: ImmutableTeams,
    pub deck: Deck,
}

impl From<PrivateGrandTichu> for PublicGrandTichu {
    fn from(item: PrivateGrandTichu) -> Self {
        PublicGrandTichu {
            grand_tichus: item.grand_tichus.clone(),
            small_tichus: item.small_tichus.clone(),
            teams: item.teams,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct CardTrade {
    from_user_id: String,
    card: Card,
    to_user_id: String,
}

/// Server state: includes sensitive information, such as the Deck & Trades
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct PrivateTrade {
    pub small_tichus: [UserIdWithTichuCallStatus; 4],
    pub grand_tichus: [UserIdWithTichuCallStatus; 4],
    pub teams: ImmutableTeams,
    pub deck: Deck,
    pub trades: [Option<CardTrade>; 4],
}

impl From<PrivateTrade> for PublicTrade {
    fn from(item: PrivateTrade) -> Self {
        PublicTrade {
            small_tichus: item.small_tichus.clone(),
            grand_tichus: item.grand_tichus.clone(),
            teams: item.teams.clone(),
        }
    }
}

impl From<PrivateGrandTichu> for PrivateTrade {
    fn from(private_grand_tichu: PrivateGrandTichu) -> Self {
        PrivateTrade {
            deck: private_grand_tichu.deck.clone(),
            grand_tichus: private_grand_tichu.grand_tichus.clone(),
            small_tichus: private_grand_tichu.small_tichus.clone(),
            teams: private_grand_tichu.teams.clone(),
            trades: [None, None, None, None],
        }
    }
}

/// Client state: does NOT include sensitive information, such as the Deck & Trades
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct PublicTrade {
    pub small_tichus: [UserIdWithTichuCallStatus; 4],
    pub grand_tichus: [UserIdWithTichuCallStatus; 4],
    pub teams: ImmutableTeams,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum PrivateGameStage {
    Lobby,
    Teams(MutableTeams),
    GrandTichu(Box<PrivateGrandTichu>),
    Trade(Box<PrivateTrade>),
    Game,
    Scoreboard,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum PublicGameStage {
    Lobby,
    Teams(MutableTeams),
    GrandTichu(Box<PublicGrandTichu>),
    Trade(Box<PublicTrade>),
    Game,
    Scoreboard,
}

impl From<PrivateGameStage> for PublicGameStage {
    fn from(item: PrivateGameStage) -> Self {
        match item {
            PrivateGameStage::Lobby => PublicGameStage::Lobby,
            PrivateGameStage::Teams(teams_state) => PublicGameStage::Teams(teams_state),
            PrivateGameStage::GrandTichu(private_grand_tichu) => {
                Self::GrandTichu(Box::new((*private_grand_tichu).into()))
            }
            PrivateGameStage::Trade(private_trade) => {
                Self::Trade(Box::new((*private_trade).into()))
            }
            PrivateGameStage::Game => PublicGameStage::Game,
            PrivateGameStage::Scoreboard => PublicGameStage::Scoreboard,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct PublicGameState {
    pub game_id: String,
    pub game_code: String,
    pub owner_id: String,
    pub stage: PublicGameStage,
    pub participants: Vec<PublicUser>,

    // unique to PublicGameState:
    pub current_user: PrivateUser,
}

/// Game state that only the server knows about.
/// This includes every user's cards, what is in the rest of the deck, etc.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct PrivateGameState {
    pub game_id: String,
    pub game_code: String,
    pub owner_id: String,
    pub stage: PrivateGameStage,
    pub participants: Vec<PrivateUser>,
    // active_player: String,
    // card_wished_for: Card,
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
            display_name: owner_display_name,
            user_id: owner_id.clone(),
            role: UserRole::Owner,
            tricks: vec![],
            hand: vec![],
            has_played_first_card: false,
        };
        PrivateGameState {
            game_id: Uuid::new_v4().to_string(),
            game_code: get_new_game_code(existing_game_codes),
            stage: PrivateGameStage::Lobby,
            participants: vec![owner_user],
            owner_id,
        }
    }

    pub fn add_user(&self, user_id: String, display_name: String) -> PrivateGameState {
        let current_participants = self.participants.len();
        let game_has_max_participants = current_participants == 4;
        let is_lobby = matches!(self.stage, PrivateGameStage::Lobby);

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
            has_played_first_card: false,
        };
        let mut new_participants = self.participants.clone();
        new_participants.push(participant);

        // if 4 have joined, the new game stage should become Teams
        let new_stage = if current_participants == 3 {
            let team_a = MutableTeam {
                id: Uuid::new_v4().to_string(),
                score: 0,
                team_name: "Team A".into(),
                user_ids: vec![user_id, self.participants.get(0).unwrap().user_id.clone()],
            };

            let team_b = MutableTeam {
                id: Uuid::new_v4().to_string(),
                score: 0,
                team_name: "Team B".into(),
                user_ids: vec![
                    self.participants.get(1).unwrap().user_id.clone(),
                    self.participants.get(2).unwrap().user_id.clone(),
                ],
            };

            PrivateGameStage::Teams([team_a, team_b])
        } else {
            PrivateGameStage::Lobby
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
                has_played_first_card: private_participant.has_played_first_card,
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
            stage: self.stage.clone().into(),
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
            PrivateGameStage::Teams(teams) => {
                //if user is on the team they want to move to already, return
                let new_team = match team_to_move_to {
                    TeamOption::TeamA => &teams[0],
                    TeamOption::TeamB => &teams[1],
                };
                if !new_team
                    .user_ids
                    .iter()
                    .any(|user_id| **user_id == *current_user_id)
                {
                    // remove user from team they were on before
                    let prev_team = match team_to_move_to {
                        TeamOption::TeamA => &mut teams[1],
                        TeamOption::TeamB => &mut teams[0],
                    };
                    prev_team
                        .user_ids
                        .retain(|user_id| user_id != current_user_id);

                    // add user to the new team
                    let new_team = match team_to_move_to {
                        TeamOption::TeamA => &mut teams[0],
                        TeamOption::TeamB => &mut teams[1],
                    };
                    new_team.user_ids.push(current_user_id.to_string());
                }
                new_state
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
            PrivateGameStage::Teams(teams) => {
                // user is on opposite team, so can't rename this team
                let opposite_team = match team_to_rename {
                    TeamOption::TeamA => &teams[1],
                    TeamOption::TeamB => &teams[0],
                };
                if !opposite_team
                    .user_ids
                    .iter()
                    .any(|user_id| **user_id == *current_user_id)
                {
                    // rename intended team
                    let team_to_rename = match team_to_rename {
                        TeamOption::TeamA => &mut teams[0],
                        TeamOption::TeamB => &mut teams[1],
                    };
                    team_to_rename.team_name = new_team_a_name.to_string();
                }
                new_state
            }
            // game stage is not teams, can't rename any team
            _ => new_state,
        }
    }

    /// Move from Teams stage to Grand Tichu stage
    pub fn start_grand_tichu(&self, requesting_user_id: &str) -> PrivateGameState {
        let mut new_game_state = self.clone();

        // requesting user must be the owner
        if new_game_state.owner_id != requesting_user_id {
            eprintln!(
                "User {} cannot start Grand Tichu stage because the user is not the owner. Ignoring request.",
                requesting_user_id,
            );
            return new_game_state;
        }

        match &new_game_state.stage {
            PrivateGameStage::Teams(teams_state) => {
                if teams_state[0].user_ids.len() == 2 && teams_state[1].user_ids.len() == 2 {
                    // participants are ready to move to game

                    // convert mutable teams to immutable teams
                    let team_0: Option<ImmutableTeam> = teams_state[0].clone().try_into().ok();
                    let team_1: Option<ImmutableTeam> = teams_state[1].clone().try_into().ok();

                    match (team_0, team_1) {
                        (Some(team_0), Some(team_1)) => {
                            let mut deck = Deck::new().shuffle().to_owned();

                            // deal 9 cards to each player
                            new_game_state
                                .participants
                                .iter_mut()
                                .for_each(|participant| {
                                    let mut cards = deck.draw(9);
                                    cards.sort();
                                    for card in cards.into_iter() {
                                        participant.hand.push(card);
                                    }
                                });

                            // create undecided Grand Tichu statuses
                            let grand_tichus = [
                                UserIdWithTichuCallStatus {
                                    user_id: new_game_state
                                        .participants
                                        .get(0)
                                        .unwrap()
                                        .user_id
                                        .clone(),
                                    tichu_call_status: TichuCallStatus::Undecided,
                                },
                                UserIdWithTichuCallStatus {
                                    user_id: new_game_state
                                        .participants
                                        .get(1)
                                        .unwrap()
                                        .user_id
                                        .clone(),
                                    tichu_call_status: TichuCallStatus::Undecided,
                                },
                                UserIdWithTichuCallStatus {
                                    user_id: new_game_state
                                        .participants
                                        .get(2)
                                        .unwrap()
                                        .user_id
                                        .clone(),
                                    tichu_call_status: TichuCallStatus::Undecided,
                                },
                                UserIdWithTichuCallStatus {
                                    user_id: new_game_state
                                        .participants
                                        .get(3)
                                        .unwrap()
                                        .user_id
                                        .clone(),
                                    tichu_call_status: TichuCallStatus::Undecided,
                                },
                            ];
                            // create undecided Small Tichu statuses
                            let small_tichus = [
                                UserIdWithTichuCallStatus {
                                    user_id: new_game_state
                                        .participants
                                        .get(0)
                                        .unwrap()
                                        .user_id
                                        .clone(),
                                    tichu_call_status: TichuCallStatus::Undecided,
                                },
                                UserIdWithTichuCallStatus {
                                    user_id: new_game_state
                                        .participants
                                        .get(1)
                                        .unwrap()
                                        .user_id
                                        .clone(),
                                    tichu_call_status: TichuCallStatus::Undecided,
                                },
                                UserIdWithTichuCallStatus {
                                    user_id: new_game_state
                                        .participants
                                        .get(2)
                                        .unwrap()
                                        .user_id
                                        .clone(),
                                    tichu_call_status: TichuCallStatus::Undecided,
                                },
                                UserIdWithTichuCallStatus {
                                    user_id: new_game_state
                                        .participants
                                        .get(3)
                                        .unwrap()
                                        .user_id
                                        .clone(),
                                    tichu_call_status: TichuCallStatus::Undecided,
                                },
                            ];

                            let grand_tichu_game_state = PrivateGrandTichu {
                                grand_tichus,
                                small_tichus,
                                teams: [team_0, team_1],
                                deck,
                            };

                            // move into Grand Tichu stage
                            new_game_state.stage =
                                PrivateGameStage::GrandTichu(Box::new(grand_tichu_game_state));

                            new_game_state
                        }
                        _ => {
                            eprintln!(
                                "Could not convert MutableTeams to ImmutableTeams. Ignoring request to start Grand Tichu stage by user {}",
                                requesting_user_id,
                            );
                            new_game_state
                        }
                    }
                } else {
                    eprintln!(
                        "Teams are not ready to start game. Ignoring request to start Grand Tichu stage by user {}",
                        requesting_user_id,
                    );
                    new_game_state
                }
            }
            _ => {
                eprintln!(
                    "Game stage is not currently teams. Ignoring request to start by user {}",
                    requesting_user_id,
                );
                new_game_state
            }
        }
    }

    pub fn call_grand_tichu(
        &self,
        call_grand_tichu_request: &CallGrandTichuRequest,
        user_id: &str,
    ) -> PrivateGameState {
        let mut new_game_state = self.clone();

        // game stage must be GrandTichu
        match &mut new_game_state.stage {
            PrivateGameStage::GrandTichu(grand_tichu_state) => {
                let i = grand_tichu_state
                    .grand_tichus
                    .iter()
                    .position(|user_call_status| *user_call_status.user_id == *user_id);
                match i {
                    None => {
                        eprintln!("Couldn't find user's call status in GrandTichu call stage. Ignoring request to call Grand Tichu from user {}", user_id);
                        return new_game_state;
                    }
                    Some(i) => {
                        let grand_tichus = &mut grand_tichu_state.grand_tichus;
                        let user_call_status = &grand_tichus[i];
                        if user_call_status.tichu_call_status != TichuCallStatus::Undecided {
                            eprintln!("User has already declared or declined Grand Tichu. Ignoring request to call Grand Tichu from user {}", user_id);
                            return new_game_state;
                        }
                        grand_tichus[i] = UserIdWithTichuCallStatus {
                            user_id: user_id.to_string(),
                            tichu_call_status: match call_grand_tichu_request {
                                CallGrandTichuRequest::Call => TichuCallStatus::Called,
                                CallGrandTichuRequest::Decline => TichuCallStatus::Declined,
                            },
                        };

                        // if this is the 4th Grand Tichu called, move game stage to Trade stage
                        let mut grand_tichus_called = 0u8;
                        for grand_tichu in grand_tichus {
                            if let TichuCallStatus::Called | TichuCallStatus::Declined =
                                grand_tichu.tichu_call_status
                            {
                                grand_tichus_called += 1;
                            }
                        }
                        if grand_tichus_called >= 4 {
                            new_game_state = new_game_state.start_trade()
                        }
                    }
                }
            }
            _ => {
                eprintln!("Can't call Grand Tichu when game stage is not GrandTichu. Ignoring request from user {}", user_id);
                return new_game_state;
            }
        }

        new_game_state
    }

    pub fn call_small_tichu(&self, user_id: &str) -> PrivateGameState {
        let mut new_game_state = self.clone();

        // game stage cannot be lobby, teams, or scoreboard
        let small_tichus = match &mut new_game_state.stage {
            PrivateGameStage::Lobby | PrivateGameStage::Teams(_) | PrivateGameStage::Scoreboard => {
                eprintln!(
                    "Can't call Small Tichu when game is not active. Ignoring request from user {}",
                    user_id
                );
                return new_game_state;
            }
            PrivateGameStage::GrandTichu(grand_tichu_state) => &mut grand_tichu_state.small_tichus,
            // TODO: add other game stages here
            _ => {
                eprintln!("Can't call Small Tichu when game stage is not GrandTichu (TODO: update once other stages are implemented). Ignoring request from user {}", user_id);
                return new_game_state;
            }
        };

        let i = small_tichus
            .iter()
            .position(|user_call_status| *user_call_status.user_id == *user_id);
        match i {
            None => {
                eprintln!("Couldn't find user's call status in call stage state. Ignoring request to call Small Tichu from user {}", user_id);
                return new_game_state;
            }
            Some(i) => {
                let user_call_status = &small_tichus[i];
                if user_call_status.tichu_call_status != TichuCallStatus::Undecided {
                    eprintln!("User is not in Undecided state about Small Tichu. Ignoring request to call Small Tichu from user {}", user_id);
                    return new_game_state;
                }
                small_tichus[i] = UserIdWithTichuCallStatus {
                    user_id: user_id.to_string(),
                    tichu_call_status: TichuCallStatus::Called,
                };
            }
        }

        new_game_state
    }

    // occurs automatically after last Grand Tichu is either Called or Denied
    fn start_trade(&self) -> PrivateGameState {
        eprintln!("Moving game stage from GrandTichu to to Trade");
        let mut new_game_state = self.clone();

        // must currently be in Grand Tichu stage
        if let PrivateGameStage::GrandTichu(mut grand_tichu) = new_game_state.stage {
            // deal the rest of the 9 cards
            for participant in new_game_state.participants.iter_mut() {
                let mut drawn_cards = grand_tichu.deck.draw(9);
                for _ in 0..drawn_cards.len() {
                    let drawn_card = drawn_cards.pop().unwrap();
                    participant.hand.push(drawn_card)
                }
            }

            // move game stage to Trade game stage
            new_game_state.stage = PrivateGameStage::Trade(Box::new((*grand_tichu).into()));
        } else {
            eprintln!("Can't start trade when not in Grand Tichu stage");
        }
        new_game_state
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum CardValue {
    Start,
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

impl Iterator for CardValue {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        let next_value = match &self {
            CardValue::Start => Some(CardValue::_2),
            CardValue::_2 => Some(CardValue::_3),
            CardValue::_3 => Some(CardValue::_4),
            CardValue::_4 => Some(CardValue::_5),
            CardValue::_5 => Some(CardValue::_6),
            CardValue::_6 => Some(CardValue::_7),
            CardValue::_7 => Some(CardValue::_8),
            CardValue::_8 => Some(CardValue::_9),
            CardValue::_9 => Some(CardValue::_10),
            CardValue::_10 => Some(CardValue::J),
            CardValue::J => Some(CardValue::Q),
            CardValue::Q => Some(CardValue::K),
            CardValue::K => Some(CardValue::A),
            CardValue::A => None,
        };
        if let Some(next_value) = &next_value {
            *self = next_value.clone()
        }
        next_value
    }
}

/// Enum of every possible card in Tichu. Iterable
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Card {
    Sword(CardValue),
    Jade(CardValue),
    Pagoda(CardValue),
    Star(CardValue),
    MahJong,
    Dog,
    Phoenix,
    Dragon,
}

impl Card {
    pub fn start_iter() -> Card {
        Card::Sword(CardValue::Start)
    }
}

impl Iterator for Card {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        let next_card = match &self {
            Card::Sword(card_value) => {
                let next_card_value = card_value.clone().next();
                match next_card_value {
                    Some(next_card_value) => Some(Card::Sword(next_card_value)),
                    None => Some(Card::Jade(CardValue::_2)),
                }
            }
            Card::Jade(card_value) => {
                let next_card_value = card_value.clone().next();
                match next_card_value {
                    Some(next_card_value) => Some(Card::Jade(next_card_value)),
                    None => Some(Card::Pagoda(CardValue::_2)),
                }
            }
            Card::Pagoda(card_value) => {
                let next_card_value = card_value.clone().next();
                match next_card_value {
                    Some(next_card_value) => Some(Card::Pagoda(next_card_value)),
                    None => Some(Card::Star(CardValue::_2)),
                }
            }
            Card::Star(card_value) => {
                let next_card_value = card_value.clone().next();
                match next_card_value {
                    Some(next_card_value) => Some(Card::Star(next_card_value)),
                    None => Some(Card::MahJong),
                }
            }
            Card::MahJong => Some(Card::Dog),
            Card::Dog => Some(Card::Phoenix),
            Card::Phoenix => Some(Card::Dragon),
            Card::Dragon => None,
        };

        if let Some(next_card) = &next_card {
            *self = next_card.clone();
        }
        next_card
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Deck(Vec<Card>);

impl Default for Deck {
    fn default() -> Self {
        let mut cards = Vec::with_capacity(56);

        for card in Card::start_iter() {
            cards.push(card);
        }

        Deck(cards)
    }
}

impl Deck {
    /// Creates a new, full, sorted Deck (i.e. it is NOT shuffled)
    pub fn new() -> Deck {
        Deck::default()
    }

    pub fn shuffle(&mut self) -> &mut Self {
        let pseudo_rand_num = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as u64;
        self.0
            .shuffle(&mut SmallRng::seed_from_u64(pseudo_rand_num));
        self
    }

    pub fn draw(&mut self, number: usize) -> Vec<Card> {
        // limit draws to size of deck
        let number = if number > self.0.len() {
            self.0.len()
        } else {
            number
        };

        let mut cards = Vec::with_capacity(number);
        for _ in 0..number {
            let popped_card = self.0.pop();
            if let Some(popped_card) = popped_card {
                cards.push(popped_card);
            }
        }

        cards
    }
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
    pub has_played_first_card: bool,
}

/// Contains private user data, including the user's hand, etc.
/// This data should be shared carefully to mitigate cheating.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct PrivateUser {
    pub user_id: String,
    pub role: UserRole,
    pub display_name: String,
    pub tricks: Vec<Vec<Card>>,
    pub has_played_first_card: bool,
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct RenameTeam {
    pub team_option: TeamOption,
    pub team_name: String,
}

/// Available options when a user either calls or declines Grand Tichu
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum CallGrandTichuRequest {
    Call,
    Decline,
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
    MoveToTeam(TeamOption),
    RenameTeam(RenameTeam),

    /// Move from Teams stage to Grand Tichu stage
    StartGrandTichu,

    CallGrandTichu(CallGrandTichuRequest),
    CallSmallTichu,

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
    DealFinalCards,

    /// Deal last 5 cards.
    /// Player with the Mah Jong leads.
    CardsTraded,

    /// after all submitted
    CardsPlayed,
    DragonWasWon,
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
