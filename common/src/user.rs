use crate::Card;
use serde::{Deserialize, Serialize};

/// Placeholder for when a user has not been assigned an id yet
pub const NO_USER_ID: &str = "NO_USER_ID";

/// Possible roles for a user in the room.
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

impl From<PrivateUser> for PublicUser {
    fn from(private_user: PrivateUser) -> PublicUser {
        PublicUser {
            display_name: private_user.display_name.clone(),
            has_played_first_card: private_user.has_played_first_card.clone(),
            role: private_user.role.clone(),
            tricks: private_user.tricks.clone(),
            user_id: private_user.user_id.clone(),
        }
    }
}

impl From<&PrivateUser> for PublicUser {
    fn from(private_user: &PrivateUser) -> PublicUser {
        PublicUser {
            display_name: private_user.display_name.clone(),
            has_played_first_card: private_user.has_played_first_card.clone(),
            role: private_user.role.clone(),
            tricks: private_user.tricks.clone(),
            user_id: private_user.user_id.clone(),
        }
    }
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
