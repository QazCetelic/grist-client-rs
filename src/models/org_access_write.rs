use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrgAccessWrite {
    #[serde(rename = "users")]
    pub users: Users,
}

impl OrgAccessWrite {
    pub fn new(users: Users) -> OrgAccessWrite {
        OrgAccessWrite {
            users,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Users {
    #[serde(rename = "owners")]
    Owners,
    #[serde(rename = "editors")]
    Editors,
    #[serde(rename = "viewers")]
    Viewers,
    #[serde(rename = "members")]
    Members,
}

impl Default for Users {
    fn default() -> Users {
        Self::Owners
    }
}

