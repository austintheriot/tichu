use crate::{Card, Deck, GetSmallTichu, ImmutableTeams, PrivateGrandTichu, SmallTichuArray};
use serde::{Deserialize, Serialize};

/// Server state: includes sensitive information, such as the Deck & Trades
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct PrivateTrade {
    pub small_tichus: SmallTichuArray,
    pub grand_tichus: SmallTichuArray,
    pub teams: ImmutableTeams,
    pub deck: Deck,
    pub trades: [Option<SubmitTrade>; 4],
}

impl From<PrivateTrade> for PublicTrade {
    fn from(private_trade: PrivateTrade) -> Self {
        // saves public trades as user_ids of those who have successfully submitted their trades
        let mut submitted_trades = Vec::new();
        for trade in private_trade.trades {
            if let Some(trade) = trade {
                let from_user_id = &trade[0].from_user_id;
                submitted_trades.push(from_user_id.clone())
            }
        }

        PublicTrade {
            small_tichus: private_trade.small_tichus.clone(),
            grand_tichus: private_trade.grand_tichus.clone(),
            teams: private_trade.teams,
            submitted_trades,
        }
    }
}

/// Transition server state from Grand Tichu stage to Trade stage
impl From<PrivateGrandTichu> for PrivateTrade {
    fn from(private_grand_tichu: PrivateGrandTichu) -> Self {
        PrivateTrade {
            deck: private_grand_tichu.deck.clone(),
            grand_tichus: private_grand_tichu.grand_tichus.clone(),
            small_tichus: private_grand_tichu.small_tichus.clone(),
            teams: private_grand_tichu.teams,
            trades: [None, None, None, None],
        }
    }
}

impl PrivateTrade {
    pub fn get_users_in_turn_order(&self) -> Vec<String> {
        vec![
            self.teams[0].user_ids[0].clone(),
            self.teams[1].user_ids[0].clone(),
            self.teams[0].user_ids[1].clone(),
            self.teams[1].user_ids[1].clone(),
        ]
    }
}

impl GetSmallTichu for PrivateTrade {
    fn get_small_tichu(&self) -> &SmallTichuArray {
        &self.small_tichus
    }

    fn get_small_tichu_mut(&mut self) -> &mut SmallTichuArray {
        &mut self.small_tichus
    }
}

/// Client state: does NOT include sensitive information, such as the Deck & Trades
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct PublicTrade {
    pub small_tichus: SmallTichuArray,
    pub grand_tichus: SmallTichuArray,
    pub teams: ImmutableTeams,

    /// If a user_id is present here, it indicates that the user
    /// has successfully submitted a trade.
    pub submitted_trades: Vec<String>,
}

impl GetSmallTichu for PublicTrade {
    fn get_small_tichu(&self) -> &SmallTichuArray {
        &self.small_tichus
    }
    fn get_small_tichu_mut(&mut self) -> &mut SmallTichuArray {
        &mut self.small_tichus
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct CardTrade {
    pub from_user_id: String,
    pub card: Card,
    pub to_user_id: String,
}

/// This is the format that the client uses to submit a card trade
/// to the server in the Trade stage. One CardTrade for each of the other
/// participants: Opponent 1, Teammate, and Opponent 2.
pub type SubmitTrade = [CardTrade; 3];
