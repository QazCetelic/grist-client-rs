use crate::models;
use serde::{Deserialize, Serialize};
use crate::models::primitive_types::GristId;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "id")]
    pub id: GristId,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "picture", deserialize_with = "Option::deserialize")]
    pub picture: Option<String>,
}

impl User {
    pub fn new(id: GristId, name: String, picture: Option<String>) -> User {
        User {
            id,
            name,
            picture,
        }
    }
}

