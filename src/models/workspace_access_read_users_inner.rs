use crate::models;
use serde::{Deserialize, Serialize};
use crate::models::primitive_types::GristId;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceAccessReadUsersInner {
    #[serde(rename = "id")]
    pub id: GristId,
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
    pub fn new(id: GristId, name: String) -> WorkspaceAccessReadUsersInner {
        WorkspaceAccessReadUsersInner {
            id,
            name,
            email: None,
            access: None,
            parent_access: None,
        }
    }
}

