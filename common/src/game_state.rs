use crate::{
    get_card_combination, get_new_game_code, next_combo_beats_prev, sort_cards_for_hand,
    user::UserRole, CallGrandTichuRequest, Card, CardSuit, Deck, GetSmallTichu, ImmutableTeam,
    MutableTeam, OtherPlayers, PrivateGameStage, PrivateGrandTichu, PrivatePlay, PrivateUser,
    PublicGameStage, PublicUser, SubmitTrade, TeamCategories, TeamOption, TichuCallStatus,
    UserIdWithTichuCallStatus, NUM_CARDS_AFTER_GRAND_TICHU, NUM_CARDS_BEFORE_GRAND_TICHU,
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

    /// Removes a uer from the game.
    pub fn remove_user(&self, user_id: &str) -> PrivateGameState {
        let mut new_participants = self.participants.clone();

        // filter out removed user
        new_participants.retain(|user| user.user_id != user_id);

        // clone old game state and update only what's necessary
        let mut new_game_state = self.clone();
        new_game_state.participants = new_participants;
        new_game_state
    }

    /// Reassigns ownership of the game to a different user.
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
            let public_participant: PublicUser = private_participant.into();
            public_participants.push(public_participant);

            if private_participant.user_id == current_user_id {
                current_user = Some(private_participant.clone());
            }
        }

        let mut current_user = if let Some(current_user) = current_user {
            current_user
        } else {
            eprintln!("Can't convert PrivateGameState to PublicGameState, because current user does not exist in list of participants");
            return None;
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

        Some(public_game_state)
    }

    /// Moves a user into a specific team.
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

    /// Renames a team
    ///
    /// Can be requested by any user who is currently on the team they
    /// are trying to rename.
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

    /// Saves a users Grand Tichu choice
    ///
    /// User can either Call or Decline
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

    /// Saves user's Small Tichu choice
    ///
    /// User can only CALL small tichu. Cannot decline.
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

    /// Start trade occurs automatically after last Grand Tichu is either Called or Denied
    /// Mutates self rather than cloning game state, since it only occurs in conjunction with CallGrandTichu
    fn start_trade(mut self) -> Self {
        eprintln!("Moving game stage from GrandTichu to to Trade");

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
            eprintln!("Can't start trade when not in Grand Tichu stage");
        }
        self
    }

    /// Saves a user's trade choice.
    ///
    /// These trades are actually committed/enacted once all users have submitted their trades.
    pub fn submit_trade(&self, user_id: &str, submit_trade: &SubmitTrade) -> PrivateGameState {
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
                eprintln!("Couldn't accept traded submitted by user {user_id} because user could not be found in participants");
                return new_game_state;
            };

            for trade in submit_trade {
                // User must actually have those cards in their hand
                if !user.hand.iter().any(|card| *card == trade.card) {
                    eprintln!("Couldn't accept traded submitted by user {} because user does {:?}, which they are trying to trade", user_id, trade.card);
                    return new_game_state;
                }

                // Trade must not be to self
                if trade.to_user_id == user_id {
                    eprintln!("Couldn't accept traded submitted by user {} because user is trying to trade to self", user_id);
                    return new_game_state;
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
                    eprintln!("Couldn't accept traded submitted by user {} because the person the user is trying to trade to was not found in the teams", user_id);
                    return new_game_state;
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
                eprintln!("Couldn't accept traded submitted by user {} because no free index was found to save to in the Trade state `trades` array", user_id);
                return new_game_state;
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
                                    eprintln!(
                                        "Game state error: Couldn't find user to trade card to"
                                    );
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
                        panic!("Could not find MahJong among the participants' hands");
                    };
                    let mut play_state: PrivatePlay = (**private_trade).clone().into();
                    play_state.turn_user_id = first_turn_user_id;
                    new_game_state.stage = PrivateGameStage::Play(Box::new(play_state));
                }
            }

            new_game_state
        } else {
            eprintln!(
                "Couldn't accept traded submitted by user {} because Game Stage is not Trade",
                user_id
            );
            new_game_state
        }
    }

    pub fn pass(&self, _user_id: &str, _user_id_to_give_dragon_to: String) -> Self {
        

        // if this is the final pass, user wins trick
        // if won and it contains a dragon, give trick to the user who it is intended for

        // else if not the final pass, merely save the pass and move the turn

        self.clone()
    }

    pub fn play_cards(
        &self,
        user_id: &str,
        next_cards: Vec<Card>,
        wished_for_card: Option<Card>,
    ) -> Self {
        let mut new_game_state = self.clone();

        // must be play stage
        if let PrivateGameStage::Play(play_stage) = &mut new_game_state.stage {
            let next_combo = get_card_combination(play_stage.table.last(), &next_cards);
            if let Some(next_combo) = next_combo {
                // must be the player's turn (unless a bomb)
                if play_stage.turn_user_id == user_id {
                    let prev_combo = play_stage.table.last();
                    // must be a valid play based on the previous card (or no card)
                    if next_combo_beats_prev(&prev_combo, &next_combo) {
                        // if there is a wish and the user can play it, does this combo contain it?
                        if let Some(wished_for_card) = &play_stage.wished_for_card {
                            let user = new_game_state
                                .participants
                                .iter()
                                .find(|user| user.user_id == user_id);
                            if let Some(user) = user {
                                let user_can_play_wish =
                                    user.hand.iter().any(|card| card == wished_for_card);
                                if user_can_play_wish {
                                    let combo_contains_wish = next_combo
                                        .cards()
                                        .iter()
                                        .any(|card| card == wished_for_card);

                                    if combo_contains_wish {
                                        // player is playing wish, so erase wished-for card
                                        play_stage.wished_for_card = None;
                                    } else {
                                        eprintln!(
                                            "Couldn't accept card play submitted by user {} because user can play wished-for card but didn't",
                                            user_id
                                        );
                                        return self.clone();
                                    }
                                }
                            } else {
                                eprintln!(
                                    "Couldn't accept card play submitted by user {} because user couldn't be found in participants",
                                    user_id
                                );
                            }
                        }

                        // put combo on table
                        play_stage.table.push(next_combo);

                        // user has now definitely played first card
                        let current_user_i = new_game_state
                            .participants
                            .iter()
                            .position(|participant| participant.user_id == user_id);
                        if let Some(current_user_i) = current_user_i {
                            new_game_state.participants[current_user_i].has_played_first_card =
                                true;
                        }

                        // user is now the winning user
                        play_stage.winning_user_id.replace(user_id.to_string());

                        // if user has wished for a card, save it
                        if let Some(wished_for_card) = wished_for_card {
                            play_stage.wished_for_card.replace(wished_for_card);
                        }

                        // move turn to next user
                        play_stage.turn_user_id = play_stage.get_next_turn_user_id().clone();
                    } else {
                        eprintln!(
                            "Couldn't accept card play submitted by user {} because combo does not beat combo on the table",
                            user_id
                        );
                    }
                } else {
                    eprintln!(
                        "Couldn't accept card play submitted by user {} because it is not the user's turn",
                        user_id
                    );
                }
            } else {
                eprintln!(
                    "Couldn't accept card play submitted by user {} because cards are not a valid combination",
                    user_id
                );
                return new_game_state;
            }
        } else {
            eprintln!(
                "Couldn't accept card play submitted by user {} because Game Stage is not Play",
                user_id
            );
            return new_game_state;
        }
        new_game_state
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
    pub fn get_mutable_team_categories(&self) -> TeamCategories<&MutableTeam> {
        if let PublicGameStage::Teams(mutable_teams) = &self.stage {
            let current_team = mutable_teams.iter().find(|team| {
                team.user_ids
                    .iter()
                    .any(|user_id| *user_id == self.current_user.user_id)
            });

            let opposing_team = if let Some(current_team) = current_team {
                mutable_teams
                    .iter()
                    .find(|team| *team.id != current_team.id)
            } else {
                None
            };

            TeamCategories {
                current_team,
                opposing_team,
            }
        } else {
            TeamCategories {
                current_team: None,
                opposing_team: None,
            }
        }
    }

    pub fn get_immutable_team_categories(&self) -> TeamCategories<&ImmutableTeam> {
        let immutable_teams = match &self.stage {
            PublicGameStage::Trade(trade) => &trade.teams,
            PublicGameStage::Play(play) => &play.teams,
            _ => {
                return TeamCategories {
                    current_team: None,
                    opposing_team: None,
                }
            }
        };

        let current_team = immutable_teams.iter().find(|team| {
            team.user_ids
                .iter()
                .any(|user_id| *user_id == self.current_user.user_id)
        });

        let opposing_team = if let Some(current_team) = current_team {
            immutable_teams
                .iter()
                .find(|team| *team.id != current_team.id)
        } else {
            None
        };

        TeamCategories {
            current_team,
            opposing_team,
        }
    }

    pub fn get_other_players(&self) -> Option<OtherPlayers<PublicUser>> {
        let team_categories = self.get_immutable_team_categories();

        if let TeamCategories {
            current_team: Some(current_team),
            opposing_team: Some(opposing_team),
        } = team_categories
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
