use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "picture", deserialize_with = "Option::deserialize")]
    pub picture: Option<String>,
}

impl User {
    pub fn new(id: i64, name: String, picture: Option<String>) -> User {
        User {
            id,
            name,
            picture,
        }
    }
}

