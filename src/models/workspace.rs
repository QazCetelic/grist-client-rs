use crate::models;
use serde::{Deserialize, Serialize};
use crate::models::primitive_types::GristId;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Workspace {
    #[serde(rename = "id")]
    pub id: GristId,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "access")]
    pub access: models::Access,
}

impl Workspace {
    pub fn new(id: GristId, name: String, access: models::Access) -> Workspace {
        Workspace {
            id,
            name,
            access,
        }
    }
}

