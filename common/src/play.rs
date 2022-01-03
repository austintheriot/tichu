use crate::{
    CardValue, GetSmallTichu, ImmutableTeam, ImmutableTeams, PrivateTrade, SmallTichuArray,
    TeamCategories, ValidCardCombo,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct PassWithUserId {
    pub user_id: String,
    pub passed: bool,
}

/// Server state: includes sensitive information, such as the Deck
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct PrivatePlay {
    pub small_tichus: SmallTichuArray,
    pub grand_tichus: SmallTichuArray,
    pub teams: ImmutableTeams,
    pub table: Vec<ValidCardCombo>,
    pub turn_user_id: String,

    /// User whose combo is currently winning the trick
    pub winning_user_id: Option<String>,
    pub user_id_to_give_dragon_to: Option<String>,
    pub wished_for_card_value: Option<CardValue>,
    pub passes: [PassWithUserId; 4],
    /// Users who have not run out of cards: in turn order
    pub users_in_play: Vec<String>,
}

impl From<PrivateTrade> for PrivatePlay {
    fn from(private_trade: PrivateTrade) -> Self {
        let passes = [
            PassWithUserId {
                user_id: private_trade.teams[0].user_ids[0].clone(),
                passed: false,
            },
            PassWithUserId {
                user_id: private_trade.teams[0].user_ids[1].clone(),
                passed: false,
            },
            PassWithUserId {
                user_id: private_trade.teams[1].user_ids[0].clone(),
                passed: false,
            },
            PassWithUserId {
                user_id: private_trade.teams[1].user_ids[1].clone(),
                passed: false,
            },
        ];

        let users_in_play = private_trade.get_users_in_turn_order();

        PrivatePlay {
            small_tichus: private_trade.small_tichus.clone(),
            grand_tichus: private_trade.grand_tichus.clone(),
            teams: private_trade.teams,
            table: vec![],
            // this value is set in game state on transition
            turn_user_id: String::from(""),
            winning_user_id: None,
            wished_for_card_value: None,
            user_id_to_give_dragon_to: None,
            passes,
            users_in_play,
        }
    }
}

impl PrivatePlay {
    pub fn get_turn_user_team_categories(&self) -> TeamCategories<&ImmutableTeam> {
        let current_team = self
            .teams
            .iter()
            .find(|team| {
                team.user_ids
                    .iter()
                    .any(|user_id| *user_id == self.turn_user_id)
            })
            .expect("Current team should be in state");

        let opposing_team = self
            .teams
            .iter()
            .find(|team| *team.id != current_team.id)
            .expect("Opposing team should be in state");

        TeamCategories {
            current_team,
            opposing_team,
        }
    }

    pub fn get_users_in_turn_order(&self) -> Vec<&String> {
        vec![
            &self.teams[0].user_ids[0],
            &self.teams[1].user_ids[0],
            &self.teams[0].user_ids[1],
            &self.teams[1].user_ids[1],
        ]
    }

    pub fn get_is_user_out(&self, user_id: &str) -> bool {
        self.users_in_play
            .iter()
            .all(|users_in_play_id| users_in_play_id != user_id)
    }

    /// Pattern of turns:
    /// Teammate 1 -> Opponent 1 -> Teammate 2 -> Opponent 2
    pub fn get_next_turn_user_id(&self) -> &String {
        let mut users_in_turn_order = self.get_users_in_turn_order().clone();
        let current_user_turn_id = &self.turn_user_id;
        let current_user_turn_id_index = users_in_turn_order
            .iter()
            .position(|user_id| *user_id == current_user_turn_id)
            .expect("User should be in list of participants");
        // put current user at the beginning of the array and the reset are still in order
        users_in_turn_order.rotate_left(current_user_turn_id_index);
        // remove current user
        users_in_turn_order.remove(0);
        // remove all users who are out
        let mut i = 0;
        while i < users_in_turn_order.len() {
            let user_id = users_in_turn_order.get(i).expect("User should be in Vec");
            if self.get_is_user_out(user_id) {
                users_in_turn_order.remove(i);
            } else {
                i += 1;
            }
        }

        let next_turn_user_id = users_in_turn_order
            .get(0)
            .expect("There should never be only one user left");
        next_turn_user_id
    }
}

impl GetSmallTichu for PrivatePlay {
    fn get_small_tichu(&self) -> &SmallTichuArray {
        &self.small_tichus
    }

    fn get_small_tichu_mut(&mut self) -> &mut SmallTichuArray {
        &mut self.small_tichus
    }
}

/// Client state: does NOT include sensitive information, such as the Deck
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct PublicPlay {
    pub small_tichus: SmallTichuArray,
    pub grand_tichus: SmallTichuArray,
    pub teams: ImmutableTeams,
    pub table: Vec<ValidCardCombo>,
    pub turn_user_id: String,
    pub passes: [PassWithUserId; 4],
    /// Users who have not run out of cards: in turn order
    pub users_in_play: Vec<String>,
    pub wished_for_card_value: Option<CardValue>,
}

impl GetSmallTichu for PublicPlay {
    fn get_small_tichu(&self) -> &SmallTichuArray {
        &self.small_tichus
    }

    fn get_small_tichu_mut(&mut self) -> &mut SmallTichuArray {
        &mut self.small_tichus
    }
}

impl From<PrivatePlay> for PublicPlay {
    fn from(private_play: PrivatePlay) -> Self {
        PublicPlay {
            small_tichus: private_play.small_tichus,
            grand_tichus: private_play.grand_tichus,
            teams: private_play.teams,
            table: private_play.table,
            turn_user_id: private_play.turn_user_id,
            passes: private_play.passes,
            users_in_play: private_play.users_in_play,
            wished_for_card_value: private_play.wished_for_card_value,
        }
    }
}
