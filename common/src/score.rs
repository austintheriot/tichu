use crate::{ImmutableTeams, PrivatePlay};
use serde::{Deserialize, Serialize};

/// Only public score is needed, since there is no sensitive information included (such as hand, etc.)
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Score {
    pub teams: ImmutableTeams,
}

impl From<PrivatePlay> for Score {
    fn from(private_play: PrivatePlay) -> Self {
        Score {
            teams: private_play.teams,
        }
    }
}
