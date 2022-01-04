use crate::{
    get_card_combination, get_new_game_code, get_user_can_play_wished_for_card,
    next_combo_beats_prev, sort_cards_for_hand, user::UserRole, CallGrandTichuRequest, Card,
    CardSuit, CardValue, Deck, GetSmallTichu, ImmutableTeam, MutableTeam, OtherPlayers,
    PassWithUserId, PrivateGameStage, PrivateGrandTichu, PrivatePlay, PrivateUser, PublicGameStage,
    PublicUser, SubmitTrade, TeamCategories, TeamOption, TichuCallStatus,
    UserIdWithTichuCallStatus, ValidCardCombo, MAX_CARDS_IN_HAND, NUM_CARDS_AFTER_GRAND_TICHU,
    NUM_CARDS_BEFORE_GRAND_TICHU,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// The primary game state for every game of Tichu stored on the server.
///
/// This is game state state that only the server knows about, and it
/// includes every user's cards, all cards in the deck, etc.
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
    /// Initializes a new, default PrivateGameState
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

    /// Adds a user to the game.
    pub fn add_user(&self, user_id: String, display_name: String) -> Result<Self, String> {
        let current_participants = self.participants.len();
        let game_has_max_participants = current_participants == 4;
        let is_lobby = matches!(self.stage, PrivateGameStage::Lobby);

        // don't add any more than 4 users at a time
        if !is_lobby || game_has_max_participants {
            return Err("Can't add more than 4 participants in a room at once".to_string());
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

        Ok(new_game_state)
    }

    /// Removes a uer from the game.
    pub fn remove_user(&self, user_id: &str) -> Result<Self, String> {
        let mut new_participants = self.participants.clone();

        // filter out removed user
        new_participants.retain(|user| user.user_id != user_id);

        // clone old game state and update only what's necessary
        let mut new_game_state = self.clone();
        new_game_state.participants = new_participants;

        Ok(new_game_state)
    }

    /// Reassigns ownership of the game to a different user.
    pub fn reassign_owner(&self) -> Result<Self, String> {
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
                Ok(new_game_state)
            }
            None => Err("Couldn't find new owner in list of participants".to_string()),
        }
    }

    /// Converts game state that only the server can see into state relevant for a specific user.
    pub fn to_public_game_state(&self, current_user_id: &str) -> Result<PublicGameState, String> {
        let mut public_participants: Vec<PublicUser> = Vec::with_capacity(4);
        let mut current_user = None;
        for private_participant in self.participants.iter() {
            let public_participant: PublicUser = private_participant.into();
            public_participants.push(public_participant);

            if private_participant.user_id == current_user_id {
                current_user = Some(private_participant.clone());
            }
        }

        let mut current_user = if let Some(current_user) = current_user {
            current_user
        } else {
            return Err(format!("Can't convert PrivateGameState to PublicGameState, because current user does not exist in list of participants"));
        };

        // sort users hand just in case
        sort_cards_for_hand(&mut current_user.hand);

        let public_game_state = PublicGameState {
            game_id: self.game_id.clone(),
            game_code: self.game_code.clone(),
            owner_id: self.owner_id.clone(),
            stage: self.stage.clone().into(),
            participants: public_participants,
            current_user,
        };

        Ok(public_game_state)
    }

    /// Moves a user into a specific team.
    pub fn move_to_team(
        &self,
        team_to_move_to: &TeamOption,
        current_user_id: &str,
    ) -> Result<Self, String> {
        let mut new_state = self.clone();
        match &mut new_state.stage {
            PrivateGameStage::Teams(teams) => {
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
                } else {
                    return Err(format!(
                        "Can't move {} to new team because user is already on team",
                        current_user_id
                    ));
                }
                Ok(new_state)
            }
            // game stage is not teams, can't move teams
            _ => Err(format!(
                "Can't move {} to new team because game stage is not Teams",
                current_user_id
            )),
        }
    }

    /// Renames a team
    ///
    /// Can be requested by any user who is currently on the team they
    /// are trying to rename.
    pub fn rename_team(
        &self,
        team_to_rename: &TeamOption,
        current_user_id: &str,
        new_team_a_name: &str,
    ) -> Result<Self, String> {
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
                    Ok(new_state)
                } else {
                    Err(format!(
                        "Can't rename team for {} because user is not on the team they want to rename",
                        current_user_id
                    ))
                }
            }
            // game stage is not teams, can't rename any team
            _ => Err(format!(
                "Can't rename team for {} because game stage is not Teams",
                current_user_id
            )),
        }
    }

    /// Move from Teams stage to Grand Tichu stage
    pub fn start_grand_tichu(&self, requesting_user_id: &str) -> Result<Self, String> {
        let mut new_game_state = self.clone();

        // requesting user must be the owner
        if new_game_state.owner_id != requesting_user_id {
            return Err(format!(
                "User {} cannot start Grand Tichu stage because the user is not the owner. Ignoring request.",
                requesting_user_id,
            ));
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
                                    let mut cards = deck.draw(NUM_CARDS_BEFORE_GRAND_TICHU);
                                    sort_cards_for_hand(&mut cards);
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

                            Ok(new_game_state)
                        }
                        _ => {
                            return Err(format!(
                                "Could not convert MutableTeams to ImmutableTeams. Ignoring request to start Grand Tichu stage by user {}",
                                requesting_user_id,
                            ));
                        }
                    }
                } else {
                    return Err(format!(
                        "Teams are not ready to start game. Ignoring request to start Grand Tichu stage by user {}",
                        requesting_user_id,
                    ));
                }
            }
            _ => {
                return Err(format!(
                    "Game stage is not currently teams. Ignoring request to start by user {}",
                    requesting_user_id,
                ));
            }
        }
    }

    /// Saves a users Grand Tichu choice
    ///
    /// User can either Call or Decline
    pub fn call_grand_tichu(
        &self,
        call_grand_tichu_request: &CallGrandTichuRequest,
        user_id: &str,
    ) -> Result<Self, String> {
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
                        return Err(format!("Couldn't find user's call status in GrandTichu call stage. Ignoring request to call Grand Tichu from user {}", user_id));
                    }
                    Some(i) => {
                        let grand_tichus = &mut grand_tichu_state.grand_tichus;
                        let user_call_status = &grand_tichus[i];
                        if user_call_status.tichu_call_status != TichuCallStatus::Undecided {
                            return Err(format!("User has already declared or declined Grand Tichu. Ignoring request to call Grand Tichu from user {}", user_id));
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
                            new_game_state = match new_game_state.start_trade() {
                                Ok(traded_started_state) => traded_started_state,
                                Err(error) => return Err(error),
                            }
                        }
                    }
                }
            }
            _ => {
                return Err(format!("Can't call Grand Tichu when game stage is not GrandTichu. Ignoring request from user {}", user_id));
            }
        }

        Ok(new_game_state)
    }

    /// Saves user's Small Tichu choice
    ///
    /// User can only CALL small tichu. Cannot decline.
    pub fn call_small_tichu(&self, user_id: &str) -> Result<Self, String> {
        let mut new_game_state = self.clone();

        // game stage cannot be lobby, teams, or scoreboard
        let small_tichus = match &mut new_game_state.stage {
            PrivateGameStage::Lobby | PrivateGameStage::Teams(_) | PrivateGameStage::Scoreboard => {
                return Err(format!(
                    "Can't call Small Tichu when game is not active. Ignoring request from user {}",
                    user_id
                ));
            }
            PrivateGameStage::GrandTichu(grand_tichu_state) => {
                grand_tichu_state.get_small_tichu_mut()
            }
            PrivateGameStage::Trade(trade) => trade.get_small_tichu_mut(),
            PrivateGameStage::Play(play) => play.get_small_tichu_mut(),
        };

        let i = small_tichus
            .iter()
            .position(|user_call_status| *user_call_status.user_id == *user_id);
        match i {
            None => {
                return Err(format!("Couldn't find user's call status in call stage state. Ignoring request to call Small Tichu from user {}", user_id));
            }
            Some(i) => {
                let user_call_status = &small_tichus[i];
                if user_call_status.tichu_call_status != TichuCallStatus::Undecided {
                    return Err(format!("User is not in Undecided state about Small Tichu. Ignoring request to call Small Tichu from user {}", user_id));
                }
                small_tichus[i] = UserIdWithTichuCallStatus {
                    user_id: user_id.to_string(),
                    tichu_call_status: TichuCallStatus::Called,
                };
            }
        }

        Ok(new_game_state)
    }

    /// Start trade occurs automatically after last Grand Tichu is either Called or Denied
    /// Mutates self rather than cloning game state, since it only occurs in conjunction with CallGrandTichu
    fn start_trade(mut self) -> Result<Self, String> {
        // must currently be in Grand Tichu stage
        if let PrivateGameStage::GrandTichu(mut grand_tichu) = self.stage {
            // deal the rest of the cards to each player
            for participant in self.participants.iter_mut() {
                let mut drawn_cards = grand_tichu.deck.draw(NUM_CARDS_AFTER_GRAND_TICHU);
                for _ in 0..drawn_cards.len() {
                    let drawn_card = drawn_cards.pop().unwrap();
                    participant.hand.push(drawn_card)
                }
            }

            // move game stage to Trade game stage
            self.stage = PrivateGameStage::Trade(Box::new((*grand_tichu).into()));
        } else {
            return Err(format!("Can't start trade when not in Grand Tichu stage"));
        }
        Ok(self)
    }

    /// Saves a user's trade choice.
    ///
    /// These trades are actually committed/enacted once all users have submitted their trades.
    pub fn submit_trade(&self, user_id: &str, submit_trade: &SubmitTrade) -> Result<Self, String> {
        let mut new_game_state = self.clone();

        // Must be Trade stage
        if let PrivateGameStage::Trade(trade_stage) = &mut new_game_state.stage {
            let i = new_game_state
                .participants
                .iter()
                .position(|user| *user.user_id == *user_id);

            let user = if let Some(i) = i {
                &mut new_game_state.participants[i]
            } else {
                return Err(format!("Couldn't accept traded submitted by user {user_id} because user could not be found in participants"));
            };

            for trade in submit_trade {
                // User must actually have those cards in their hand
                if !user.hand.iter().any(|card| *card == trade.card) {
                    return Err(format!("Couldn't accept traded submitted by user {} because user does {:?}, which they are trying to trade", user_id, trade.card));
                }

                // Trade must not be to self
                if trade.to_user_id == user_id {
                    return Err( format!("Couldn't accept traded submitted by user {} because user is trying to trade to self", user_id));
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
                    return Err(format!("Couldn't accept traded submitted by user {} because the person the user is trying to trade to was not found in the teams", user_id));
                }
            }

            // Save the user's trade for later
            let free_index = trade_stage
                .trades
                .iter()
                .position(|card_trade| card_trade.is_none());
            if let Some(free_index) = free_index {
                trade_stage.trades[free_index] = Some(submit_trade.clone());
            } else {
                return Err(format!("Couldn't accept traded submitted by user {} because no free index was found to save to in the Trade state `trades` array", user_id));
            }

            // Remove traded cards from user's hand
            for trade in submit_trade {
                let i = user.hand.iter().position(|card| *card == trade.card);
                if let Some(i) = i {
                    user.hand.remove(i);
                }
            }

            // Once all trades have been received, actually trade the cards and move to Game
            let mut submitted_trades: u8 = 0;
            for trade in trade_stage.trades.iter() {
                if trade.is_some() {
                    submitted_trades += 1;
                }
            }
            if submitted_trades >= 4 {
                if let PrivateGameStage::Trade(private_trade) = &new_game_state.stage {
                    // actually trade the cards
                    for trade in &private_trade.trades {
                        if let Some(trade) = trade {
                            for card in trade {
                                let i =
                                    new_game_state.participants.iter().position(|private_user| {
                                        *private_user.user_id == card.to_user_id
                                    });
                                if let Some(i) = i {
                                    new_game_state.participants[i].hand.push(card.card.clone());
                                } else {
                                    return Err(format!(
                                        "Game state error: Couldn't find user to trade card to"
                                    ));
                                }
                            }
                        }
                    }

                    // player with the MahJong leads first
                    let mut first_turn_user_id = None;
                    'outer: for user in new_game_state.participants.iter() {
                        for card in &user.hand {
                            if card.suit == CardSuit::MahJong {
                                first_turn_user_id = Some(user.user_id.clone());
                                break 'outer;
                            }
                        }
                    }
                    let first_turn_user_id = if let Some(first_turn_user_id) = first_turn_user_id {
                        first_turn_user_id
                    } else {
                        return Err("Could not find MahJong among the participants' hands".into());
                    };
                    let mut play_state: PrivatePlay = (**private_trade).clone().into();
                    play_state.turn_user_id = first_turn_user_id;
                    new_game_state.stage = PrivateGameStage::Play(Box::new(play_state));
                }
            }

            Ok(new_game_state)
        } else {
            return Err(format!(
                "Couldn't accept traded submitted by user {} because Game Stage is not Trade",
                user_id
            ));
        }
    }

    pub fn get_number_of_users_who_have_passed(&self) -> Result<usize, String> {
        if let PrivateGameStage::Play(play_state) = &self.stage {
            let passed_users: Vec<&PassWithUserId> = play_state
                .passes
                .iter()
                .filter(|pass| pass.passed)
                .collect();
            return Ok(passed_users.len());
        }

        Err(String::from(
            "Can't get number of passes when game stage is not PlayStage",
        ))
    }

    pub fn get_only_turn_users_teammates_are_in_play(&self) -> bool {
        if let PrivateGameStage::Play(play_state) = &self.stage {
            let TeamCategories { current_team, .. } = play_state.get_turn_user_team_categories();
            play_state.users_in_play.iter().all(|user_id_in_play| {
                current_team
                    .user_ids
                    .iter()
                    .any(|team_user_id| team_user_id == user_id_in_play)
            })
        } else {
            false
        }
    }

    pub fn pass(&self, user_id: &str) -> Result<Self, String> {
        let mut new_game_state = self.clone();

        let is_final_pass = new_game_state.get_number_of_users_who_have_passed() == Ok(3);

        let new_play_state =
            if let PrivateGameStage::Play(new_play_state) = &mut new_game_state.stage {
                new_play_state
            } else {
                return Err(format!(
                    "Couldn't accept pass from user {} because Game Stage is not Play",
                    user_id
                ));
            };

        // if this is the final pass, next user wins the trick,so move them into the user's tricks
        if is_final_pass {
            let last_trick = new_play_state
                .table
                .get(0)
                .expect("Table should have at least one combo if this is the final pass");
            let last_trick_contains_dragon = last_trick
                .cards()
                .iter()
                .any(|card| card.suit == CardSuit::Dragon);

            let receiving_user_id = if last_trick_contains_dragon {
                // if it contains a dragon, give trick to the user who the winner chose
                new_play_state
                .user_id_to_give_dragon_to
                .as_ref()
                .expect("If someone won with the dragon, then the user has to have picked a user to give it to")
                .clone()
            } else {
                // if it does not contains a dragon, give it to the user who played the last trick
                last_trick.user_id().clone()
            };

            // remove trick from table and give to receiving user
            let receiving_user = new_game_state
                .participants
                .iter_mut()
                .find(|user| (*user.user_id == *receiving_user_id))
                .expect("Receiving user should be in the game");
            let mut current_table_cards: Vec<ValidCardCombo> =
                new_play_state.table.drain(..).collect();
            receiving_user.tricks.append(&mut current_table_cards);

            // if round is over, get if only team is left in play
            let TeamCategories {
                current_team,
                opposing_team,
            } = new_play_state.get_turn_user_team_categories();
            let only_one_team_is_in_play =
                new_play_state.users_in_play.iter().all(|user_id_in_play| {
                    current_team
                        .user_ids
                        .iter()
                        .any(|team_user_id| team_user_id == user_id_in_play)
                }) || new_play_state.users_in_play.iter().all(|user_id_in_play| {
                    opposing_team
                        .user_ids
                        .iter()
                        .any(|team_user_id| team_user_id == user_id_in_play)
                });
            // round over (plain)
            if (new_play_state.users_in_play.len() == 1)
                // round over (double victory)
                || (new_play_state.users_in_play.len() == 2
                    && only_one_team_is_in_play)
            {
                return new_game_state.round_over();
            }
        }

        // else if not the final pass, merely save the pass and move the turn
        let users_index = new_play_state
            .passes
            .iter()
            .position(|pass| pass.user_id == user_id)
            .expect("User should be in the passes state");
        new_play_state.passes[users_index].passed = true;
        new_play_state.turn_user_id = new_play_state.get_next_turn_user_id().clone();

        Ok(new_game_state)
    }

    fn round_over(&self) -> Result<Self, String> {
        // if only one person is left then the round is over (plain over)
        // else if only 2 users who are on the same team are left, then it is a double victory

        // if point goal has been met then game is over, so move to to scoreboard
        // else start next round
        todo!();
    }

    pub fn get_user_by_user_id(&self, user_id: &str) -> Option<&PrivateUser> {
        self.participants
            .iter()
            .find(|user| (*user.user_id == *user_id))
    }

    pub fn get_user_is_out_of_cards(&self, user_id: &str) -> bool {
        let user = self
            .get_user_by_user_id(user_id)
            .expect("User should exist in game state");
        user.hand.is_empty()
    }

    pub fn __admin_skip_to_play(&self) -> Result<Self, String> {
        eprintln!("{:#?}", self);
        if let PrivateGameStage::Teams(team_state) = &self.stage {
            let mut updated_participants = self.participants.clone();
            let mut deck = Deck::new();
            deck.shuffle();
            updated_participants[0].hand = deck.0[0..MAX_CARDS_IN_HAND].to_vec();
            updated_participants[1].hand =
                deck.0[(MAX_CARDS_IN_HAND * 1)..(MAX_CARDS_IN_HAND * 2)].to_vec();
            updated_participants[2].hand =
                deck.0[(MAX_CARDS_IN_HAND * 2)..(MAX_CARDS_IN_HAND * 3)].to_vec();
            updated_participants[3].hand =
                deck.0[(MAX_CARDS_IN_HAND * 3)..(MAX_CARDS_IN_HAND * 4)].to_vec();

            let user_id_who_has_mah_jong = updated_participants
                .iter()
                .find(|participant| participant.hand.contains(&Card::mah_jong()))
                .expect("Some user should have the Mah Jong")
                .user_id
                .clone();

            let updated_teams: [ImmutableTeam; 2] = [
                team_state[0].clone().try_into().unwrap(),
                team_state[1].clone().try_into().unwrap(),
            ];

            let new_game_stage = PrivateGameStage::Play(Box::new(PrivatePlay {
                grand_tichus: [
                    UserIdWithTichuCallStatus {
                        user_id: self.participants[0].user_id.clone(),
                        tichu_call_status: TichuCallStatus::Declined,
                    },
                    UserIdWithTichuCallStatus {
                        user_id: self.participants[1].user_id.clone(),
                        tichu_call_status: TichuCallStatus::Declined,
                    },
                    UserIdWithTichuCallStatus {
                        user_id: self.participants[2].user_id.clone(),
                        tichu_call_status: TichuCallStatus::Declined,
                    },
                    UserIdWithTichuCallStatus {
                        user_id: self.participants[3].user_id.clone(),
                        tichu_call_status: TichuCallStatus::Declined,
                    },
                ],
                small_tichus: [
                    UserIdWithTichuCallStatus {
                        user_id: self.participants[0].user_id.clone(),
                        tichu_call_status: TichuCallStatus::Declined,
                    },
                    UserIdWithTichuCallStatus {
                        user_id: self.participants[1].user_id.clone(),
                        tichu_call_status: TichuCallStatus::Declined,
                    },
                    UserIdWithTichuCallStatus {
                        user_id: self.participants[2].user_id.clone(),
                        tichu_call_status: TichuCallStatus::Declined,
                    },
                    UserIdWithTichuCallStatus {
                        user_id: self.participants[3].user_id.clone(),
                        tichu_call_status: TichuCallStatus::Declined,
                    },
                ],
                teams: [
                    team_state[0].clone().try_into().unwrap(),
                    team_state[1].clone().try_into().unwrap(),
                ],
                table: Vec::new(),
                turn_user_id: user_id_who_has_mah_jong,
                winning_user_id: None,
                user_id_to_give_dragon_to: None,
                wished_for_card_value: None,
                passes: [
                    PassWithUserId {
                        passed: false,
                        user_id: "0".into(),
                    },
                    PassWithUserId {
                        passed: false,
                        user_id: "1".into(),
                    },
                    PassWithUserId {
                        passed: false,
                        user_id: "2".into(),
                    },
                    PassWithUserId {
                        passed: false,
                        user_id: "3".into(),
                    },
                ],
                users_in_play: vec![
                    updated_teams[0].user_ids[0].clone(),
                    updated_teams[1].user_ids[0].clone(),
                    updated_teams[0].user_ids[1].clone(),
                    updated_teams[1].user_ids[1].clone(),
                ],
            }));

            let new_state = PrivateGameState {
                game_code: self.game_code.clone(),
                game_id: self.game_id.clone(),
                owner_id: self.owner_id.clone(),
                participants: updated_participants,
                stage: new_game_stage,
            };

            Ok(new_state)
        } else {
            Err(String::from("Not in Teams stage"))
        }
    }

    pub fn play_cards(
        &self,
        user_id: &str,
        next_cards: Vec<Card>,
        wished_for_card_value: Option<CardValue>,
        user_id_to_give_dragon_to: Option<String>,
    ) -> Result<Self, String> {
        let mut new_game_state = self.clone();

        // must be play stage
        if let PrivateGameStage::Play(new_play_stage) = &mut new_game_state.stage {
            let next_combo =
                get_card_combination(new_play_stage.table.last(), &next_cards, user_id);
            if let Some(next_combo) = next_combo {
                let is_bomb = next_combo.is_bomb();

                // if is a bomb, then it must become that users' turn (and the others must pass as usual)
                if is_bomb {
                    new_play_stage.turn_user_id = user_id.into();
                }

                // must be the player's turn (unless a bomb)
                if new_play_stage.turn_user_id == user_id || is_bomb {
                    let prev_combo = new_play_stage.table.last();
                    // must be a valid play based on the previous card (or no card)
                    if next_combo_beats_prev(&prev_combo, &next_combo) {
                        // if there is a wish and the user can play it, does this combo contain it?
                        if let Some(wished_for_card) = &new_play_stage.wished_for_card_value {
                            let user = new_game_state
                                .participants
                                .iter()
                                .find(|user| user.user_id == user_id);
                            if user.is_some() {
                                let user_can_play_wish = get_user_can_play_wished_for_card(
                                    prev_combo,
                                    &next_cards,
                                    wished_for_card,
                                );

                                if user_can_play_wish {
                                    let combo_contains_wish = next_combo
                                        .cards()
                                        .iter()
                                        .any(|card| card.value == *wished_for_card);

                                    if combo_contains_wish {
                                        // player is playing wish, so erase wished-for card
                                        new_play_stage.wished_for_card_value = None;
                                    } else {
                                        return Err(format!(
                                            "Couldn't accept card play submitted by user {} because user can play wished-for card but didn't",
                                            user_id
                                        ));
                                    }
                                }
                            } else {
                                return Err(format!(
                                    "Couldn't accept card play submitted by user {} because user couldn't be found in participants",
                                    user_id
                                ));
                            }
                        }

                        // if user played a dragon, save who they want to give it to if they win
                        if next_cards.iter().any(|card| card.suit == CardSuit::Dragon) {
                            new_play_stage.user_id_to_give_dragon_to = user_id_to_give_dragon_to;
                        }

                        // put combo on table
                        new_play_stage.table.push(next_combo);

                        // get current user
                        let current_user_i = new_game_state
                            .participants
                            .iter()
                            .position(|participant| participant.user_id == user_id)
                            .expect("Current user should be in list of participants");

                        // user has now definitely played first card
                        let mut new_current_user = &mut new_game_state.participants[current_user_i];
                        new_current_user.has_played_first_card = true;

                        // if user is out of cards, remove them from users_in_play
                        if new_current_user.hand.is_empty() {
                            new_play_stage
                                .users_in_play
                                .retain(|user_id_in_play| *user_id_in_play == user_id)
                        }

                        // user is now the winning user
                        new_play_stage.winning_user_id.replace(user_id.to_string());

                        // if user played mahjong and has wished for a card, save it
                        let user_played_mah_jong =
                            next_cards.iter().any(|card| card.suit == CardSuit::MahJong);
                        if user_played_mah_jong {
                            new_play_stage.wished_for_card_value = wished_for_card_value;
                        }

                        // clear played cards from user's hand
                        new_current_user
                            .hand
                            .retain(|card| !next_cards.contains(card));

                        // if round is over get if only teammates are left in play
                        let TeamCategories {
                            current_team,
                            opposing_team,
                            ..
                        } = new_play_stage.get_turn_user_team_categories();
                        let only_one_team_is_in_play =
                            new_play_stage.users_in_play.iter().all(|user_id_in_play| {
                                current_team
                                    .user_ids
                                    .iter()
                                    .any(|team_user_id| team_user_id == user_id_in_play)
                            }) || new_play_stage.users_in_play.iter().all(|user_id_in_play| {
                                opposing_team
                                    .user_ids
                                    .iter()
                                    .any(|team_user_id| team_user_id == user_id_in_play)
                            });
                        // round over (plain)
                        if (new_play_stage.users_in_play.len() == 1)
                            // round over (double victory)
                            || (new_play_stage.users_in_play.len() == 2
                                && only_one_team_is_in_play)
                        {
                            return self.round_over();
                        }

                        // if we've gotten this far
                        // there should always be users left in play so, move to the next user
                        let next_user_id = new_play_stage.get_next_turn_user_id();
                        new_play_stage.turn_user_id = next_user_id.clone();
                    } else {
                        return Err(format!(
                            "Couldn't accept card play submitted by user {} because combo does not beat combo on the table",
                            user_id
                        ));
                    }
                } else {
                    return Err(format!(
                        "Couldn't accept card play submitted by user {} because it is not the user's turn",
                        user_id
                    ));
                }
            } else {
                return Err(format!(
                    "Couldn't accept card play submitted by user {} because cards are not a valid combination",
                    user_id
                ));
            }
        } else {
            return Err(format!(
                "Couldn't accept card play submitted by user {} because Game Stage is not Play",
                user_id
            ));
        }

        Ok(new_game_state)
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

impl PublicGameState {
    pub fn get_mutable_team_categories(&self) -> Option<TeamCategories<&MutableTeam>> {
        if let PublicGameStage::Teams(mutable_teams) = &self.stage {
            let current_team = mutable_teams
                .iter()
                .find(|team| {
                    team.user_ids
                        .iter()
                        .any(|user_id| *user_id == self.current_user.user_id)
                })
                .expect("Current user's team should be in state");

            let opposing_team = mutable_teams
                .iter()
                .find(|team| *team.id != current_team.id)
                .expect("Opposing team should be in state");

            Some(TeamCategories {
                current_team,
                opposing_team,
            })
        } else {
            None
        }
    }

    pub fn get_immutable_team_categories(&self) -> Option<TeamCategories<&ImmutableTeam>> {
        let immutable_teams = match &self.stage {
            PublicGameStage::Trade(trade) => &trade.teams,
            PublicGameStage::Play(play) => &play.teams,
            _ => return None,
        };

        let current_team = immutable_teams
            .iter()
            .find(|team| {
                team.user_ids
                    .iter()
                    .any(|user_id| *user_id == self.current_user.user_id)
            })
            .expect("Current user's team should be in state");

        let opposing_team = immutable_teams
            .iter()
            .find(|team| *team.id != current_team.id)
            .expect("Opposing team should be in state");

        Some(TeamCategories {
            current_team,
            opposing_team,
        })
    }

    pub fn get_other_players(&self) -> Option<OtherPlayers<PublicUser>> {
        let team_categories = self.get_immutable_team_categories();

        if let Some(TeamCategories {
            current_team,
            opposing_team,
        }) = team_categories
        {
            Some(OtherPlayers::<PublicUser> {
                opponent_1: {
                    let user_id = &opposing_team.user_ids[0];
                    self.get_user_by_user_id(user_id).unwrap().clone()
                },
                teammate: {
                    let user_id = current_team
                        .user_ids
                        .iter()
                        .find(|user_id| **user_id != *self.current_user.user_id)
                        .unwrap();
                    self.get_user_by_user_id(user_id).unwrap().clone()
                },
                opponent_2: {
                    let user_id = &opposing_team.user_ids[1];
                    self.get_user_by_user_id(user_id).unwrap().clone()
                },
            })
        } else {
            None
        }
    }

    pub fn get_user_by_user_id(&self, user_id: &str) -> Option<&PublicUser> {
        self.participants
            .iter()
            .find(|user| (*user.user_id == *user_id))
    }
}
