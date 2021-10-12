use crate::{Deck, ImmutableTeams};
use serde::{Deserialize, Serialize};

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

pub type SmallTichuArray = [UserIdWithTichuCallStatus; 4];

/// Server state: includes sensitive information, such as the Deck
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct PrivateGrandTichu {
    pub small_tichus: SmallTichuArray,
    pub grand_tichus: SmallTichuArray,
    pub teams: ImmutableTeams,
    pub deck: Deck,
}

impl GetSmallTichu for PrivateGrandTichu {
    fn get_small_tichu(&self) -> &SmallTichuArray {
        &self.small_tichus
    }

    fn get_small_tichu_mut(&mut self) -> &mut SmallTichuArray {
        &mut self.small_tichus
    }
}

/// Client state: does NOT include sensitive information, such as the Deck
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct PublicGrandTichu {
    pub small_tichus: SmallTichuArray,
    pub grand_tichus: SmallTichuArray,
    pub teams: ImmutableTeams,
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

impl GetSmallTichu for PublicGrandTichu {
    fn get_small_tichu(&self) -> &SmallTichuArray {
        &self.small_tichus
    }
    fn get_small_tichu_mut(&mut self) -> &mut SmallTichuArray {
        &mut self.small_tichus
    }
}

/// Available options when a user either calls or declines Grand Tichu.
///
/// Only Grand Tichu requires a specification of Call/Decline.
/// Small Tichu does not require the option to Decline, because Small Tichu can be
/// called freely before the user's first turn and doesn't require a pause in
/// game play to check what the user wants to do.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum CallGrandTichuRequest {
    Call,
    Decline,
}

pub trait GetSmallTichu {
    fn get_small_tichu(&self) -> &SmallTichuArray;
    fn get_small_tichu_mut(&mut self) -> &mut SmallTichuArray;
}
