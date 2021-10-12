use serde::{Deserialize, Serialize};

use crate::{PrivateUser, PublicUser};

/// Users can freely move in/out of this team type, since the
/// user_ids is a flexible Vec type.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct MutableTeam {
    pub id: String,
    pub team_name: String,
    pub user_ids: Vec<String>,
    pub score: i32,
}

/// Users cannot be easily moved in/out of ImmutableTeams, because each
/// user_ids is a fixed-length array.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct ImmutableTeam {
    pub id: String,
    pub team_name: String,
    pub user_ids: [String; 2],
    pub score: i32,
}

impl TryFrom<MutableTeam> for ImmutableTeam {
    type Error = String;
    fn try_from(item: MutableTeam) -> Result<Self, String> {
        let user_id1 = item.user_ids.get(0);
        let user_id2 = item.user_ids.get(1);
        if user_id1.is_none() || user_id2.is_none() {
            return Err("Could not convert MutableTeam to ImmutableTeam, since there were not enough user_ids".to_string());
        }

        Ok(ImmutableTeam {
            id: item.id,
            score: item.score,
            team_name: item.team_name,
            user_ids: [user_id1.unwrap().to_string(), user_id2.unwrap().to_string()],
        })
    }
}

/// Fixed-size array of 2 MutableTeam structs.
pub type MutableTeams = [MutableTeam; 2];

/// Fixed-size array of 2 ImmutableTeam structs.
pub type ImmutableTeams = [ImmutableTeam; 2];

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct PrivateOtherPlayers {
    pub opponent_1: PrivateUser,
    pub teammate: PrivateUser,
    pub opponent_2: PrivateUser,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct PublicOtherPlayers {
    pub opponent_1: PublicUser,
    pub teammate: PublicUser,
    pub opponent_2: PublicUser,
}

/// Useful for specifying which team an action is intended to be carried out on.
/// i.e. Moving teams, renaming teams, etc.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum TeamOption {
    TeamA,
    TeamB,
}

/// Enum to distinguish between other players by their relationship to the current player
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum OtherPlayerOption {
    Opponent1,
    Teammate,
    Opponent2,
}
