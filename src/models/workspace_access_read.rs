use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceAccessRead {
    #[serde(rename = "maxInheritedRole")]
    pub max_inherited_role: models::Access,
    #[serde(rename = "users")]
    pub users: Vec<models::WorkspaceAccessReadUsersInner>,
}

impl WorkspaceAccessRead {
    pub fn new(max_inherited_role: models::Access, users: Vec<models::WorkspaceAccessReadUsersInner>) -> WorkspaceAccessRead {
        WorkspaceAccessRead {
            max_inherited_role,
            users,
        }
    }
}

