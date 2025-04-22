use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Doc {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "access")]
    pub access: models::Access,
    #[serde(rename = "isPinned")]
    pub is_pinned: bool,
    #[serde(rename = "urlId", deserialize_with = "Option::deserialize")]
    pub url_id: Option<String>,
}

impl Doc {
    pub fn new(id: String, name: String, access: models::Access, is_pinned: bool, url_id: Option<String>) -> Doc {
        Doc {
            id,
            name,
            access,
            is_pinned,
            url_id,
        }
    }
}

