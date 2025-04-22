use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceAccessWrite {
    #[serde(rename = "maxInheritedRole", skip_serializing_if = "Option::is_none")]
    pub max_inherited_role: Option<models::Access>,
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<Users>,
}

impl WorkspaceAccessWrite {
    pub fn new() -> WorkspaceAccessWrite {
        WorkspaceAccessWrite {
            max_inherited_role: None,
            users: None,
        }
    }
}
/// 
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

