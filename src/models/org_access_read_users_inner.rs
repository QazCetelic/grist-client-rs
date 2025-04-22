use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrgAccessReadUsersInner {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "access", skip_serializing_if = "Option::is_none")]
    pub access: Option<models::Access>,
}

impl OrgAccessReadUsersInner {
    pub fn new(id: i32, name: String) -> OrgAccessReadUsersInner {
        OrgAccessReadUsersInner {
            id,
            name,
            email: None,
            access: None,
        }
    }
}

