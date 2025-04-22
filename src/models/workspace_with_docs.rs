use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceWithDocs {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "access")]
    pub access: models::Access,
    #[serde(rename = "docs")]
    pub docs: Vec<models::Doc>,
}

impl WorkspaceWithDocs {
    pub fn new(id: i64, name: String, access: models::Access, docs: Vec<models::Doc>) -> WorkspaceWithDocs {
        WorkspaceWithDocs {
            id,
            name,
            access,
            docs,
        }
    }
}

