use crate::Card;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum UserRole {
    Owner,
    Participant,
}

/// Contain's public information about the user:
/// Does NOT contain the user's hand.
/// This is important when sending data to other participants.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct PublicUser {
    pub user_id: String,
    pub role: UserRole,
    pub display_name: String,
    pub tricks: Vec<Vec<Card>>,
    pub has_played_first_card: bool,
}

/// Contains private user data, including the user's hand, etc.
/// This data should be shared carefully to mitigate cheating.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct PrivateUser {
    pub user_id: String,
    pub role: UserRole,
    pub display_name: String,
    pub tricks: Vec<Vec<Card>>,
    pub has_played_first_card: bool,
    pub hand: Vec<Card>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum User {
    PrivateUser(PrivateUser),
    PublicUser(PublicUser),
}
