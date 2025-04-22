use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrgAccessRead {
    #[serde(rename = "users")]
    pub users: Vec<models::OrgAccessReadUsersInner>,
}

impl OrgAccessRead {
    pub fn new(users: Vec<models::OrgAccessReadUsersInner>) -> OrgAccessRead {
        OrgAccessRead {
            users,
        }
    }
}

