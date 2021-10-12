use crate::{Deck, GetSmallTichu, ImmutableTeams, PrivateTrade, SmallTichuArray};
use serde::{Deserialize, Serialize};

/// Server state: includes sensitive information, such as the Deck
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct PrivatePlay {
    pub small_tichus: SmallTichuArray,
    pub grand_tichus: SmallTichuArray,
    pub teams: ImmutableTeams,
    pub deck: Deck,
}

impl From<PrivateTrade> for PrivatePlay {
    fn from(private_trade: PrivateTrade) -> Self {
        PrivatePlay {
            small_tichus: private_trade.small_tichus.clone(),
            grand_tichus: private_trade.grand_tichus.clone(),
            teams: private_trade.teams.clone(),
            deck: private_trade.deck.clone(),
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
            small_tichus: private_play.small_tichus.clone(),
            grand_tichus: private_play.grand_tichus.clone(),
            teams: private_play.teams.clone(),
        }
    }
}
