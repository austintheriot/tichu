use crate::{
    Card, GetSmallTichu, ImmutableTeam, ImmutableTeams, PrivateTrade, SmallTichuArray,
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
    pub wished_for_card: Option<Card>,
    pub passes: [PassWithUserId; 4],
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

        PrivatePlay {
            small_tichus: private_trade.small_tichus.clone(),
            grand_tichus: private_trade.grand_tichus.clone(),
            teams: private_trade.teams,
            table: vec![],
            // this value is set in game state on transition
            turn_user_id: String::from(""),
            winning_user_id: None,
            wished_for_card: None,
            user_id_to_give_dragon_to: None,
            passes,
        }
    }
}

impl PrivatePlay {
    pub fn get_turn_user_team_categories(&self) -> TeamCategories<&ImmutableTeam> {
        let current_team = self.teams.iter().find(|team| {
            team.user_ids
                .iter()
                .any(|user_id| *user_id == self.turn_user_id)
        });

        let opposing_team = if let Some(current_team) = current_team {
            self.teams.iter().find(|team| *team.id != current_team.id)
        } else {
            None
        };

        TeamCategories {
            current_team,
            opposing_team,
        }
    }

    /// Pattern of turns:
    /// Teammate 1 -> Opponent 1 -> Teammate 2 -> Opponent 2
    pub fn get_next_turn_user_id(&self) -> &String {
        let current_user_turn = &self.turn_user_id;
        let teammate_1 = &self.teams[0].user_ids[0];
        let opponent_1 = &self.teams[1].user_ids[0];
        let teammate_2 = &self.teams[0].user_ids[1];
        let opponent_2 = &self.teams[1].user_ids[1];

        if current_user_turn == teammate_1 {
            opponent_1
        } else if current_user_turn == opponent_1 {
            teammate_2
        } else if current_user_turn == teammate_2 {
            opponent_2
        } else {
            teammate_1
        }
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
        }
    }
}
