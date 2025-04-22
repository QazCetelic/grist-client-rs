use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Workspace {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "access")]
    pub access: models::Access,
}

impl Workspace {
    pub fn new(id: i64, name: String, access: models::Access) -> Workspace {
        Workspace {
            id,
            name,
            access,
        }
    }
}

