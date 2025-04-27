use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsersUserIdDeleteRequest {
    /// The user's name to delete (for confirmation, to avoid deleting the wrong account).
    #[serde(rename = "name")]
    pub name: String,
}

impl UsersUserIdDeleteRequest {
    pub fn new(name: String) -> UsersUserIdDeleteRequest {
        UsersUserIdDeleteRequest {
            name,
        }
    }
}

