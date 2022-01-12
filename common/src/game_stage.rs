use serde::{Deserialize, Serialize};

use crate::{
    MutableTeams, PrivateGrandTichu, PrivatePlay, PrivateTrade, PublicGrandTichu, PublicPlay,
    PublicTrade, Score,
};

/// Server state that holds all stage-specific state and is PRIVATE.
/// i.e. it contains detailed information about the deck, users' hands, etc.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum PrivateGameStage {
    Lobby,
    Teams(MutableTeams),
    GrandTichu(Box<PrivateGrandTichu>),
    Trade(Box<PrivateTrade>),
    Play(Box<PrivatePlay>),
    Score(Score),
}

/// Client state that holds all stage-specific state and is PUBLIC.
/// i.e. it does NOT contain detailed information about the deck, but does
/// contain private information about the current client's hand.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum PublicGameStage {
    Lobby,
    Teams(MutableTeams),
    GrandTichu(Box<PublicGrandTichu>),
    Trade(Box<PublicTrade>),
    Play(Box<PublicPlay>),
    Score(Score),
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
            PrivateGameStage::Play(private_play) => Self::Play(Box::new((*private_play).into())),
            PrivateGameStage::Score(score) => PublicGameStage::Score(score),
        }
    }
}

impl PublicGameStage {
    pub fn is_active(&self) -> bool {
        match &self {
            PublicGameStage::GrandTichu(_) => true,
            PublicGameStage::Trade(_) => true,
            PublicGameStage::Play(_) => true,
            _ => false,
        }
    }
}
