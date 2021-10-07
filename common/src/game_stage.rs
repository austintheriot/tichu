use serde::{Deserialize, Serialize};

use crate::{MutableTeams, PrivateGrandTichu, PrivateTrade, PublicGrandTichu, PublicTrade};

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
