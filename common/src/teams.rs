use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct MutableTeam {
    pub id: String,
    pub team_name: String,
    pub user_ids: Vec<String>,
    pub score: i32,
}

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

pub type MutableTeams = [MutableTeam; 2];

pub type ImmutableTeams = [ImmutableTeam; 2];

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum TeamOption {
    TeamA,
    TeamB,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct ChooseTeamMessage {
    pub team_id: String,
}
