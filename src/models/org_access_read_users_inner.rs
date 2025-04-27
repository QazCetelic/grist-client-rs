use crate::models;
use serde::{Deserialize, Serialize};
use crate::models::primitive_types::GristId;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrgAccessReadUsersInner {
    #[serde(rename = "id")]
    pub id: GristId,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "access", skip_serializing_if = "Option::is_none")]
    pub access: Option<models::Access>,
}

impl OrgAccessReadUsersInner {
    pub fn new(id: GristId, name: String) -> OrgAccessReadUsersInner {
        OrgAccessReadUsersInner {
            id,
            name,
            email: None,
            access: None,
        }
    }
}

