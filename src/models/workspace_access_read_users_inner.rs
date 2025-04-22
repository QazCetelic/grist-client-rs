use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceAccessReadUsersInner {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "access", skip_serializing_if = "Option::is_none")]
    pub access: Option<models::Access>,
    #[serde(rename = "parentAccess", skip_serializing_if = "Option::is_none")]
    pub parent_access: Option<models::Access>,
}

impl WorkspaceAccessReadUsersInner {
    pub fn new(id: i32, name: String) -> WorkspaceAccessReadUsersInner {
        WorkspaceAccessReadUsersInner {
            id,
            name,
            email: None,
            access: None,
            parent_access: None,
        }
    }
}

