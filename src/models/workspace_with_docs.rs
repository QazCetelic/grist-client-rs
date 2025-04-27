use crate::models;
use serde::{Deserialize, Serialize};
use crate::models::primitive_types::GristId;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceWithDocs {
    #[serde(rename = "id")]
    pub id: GristId,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "access")]
    pub access: models::Access,
    #[serde(rename = "docs")]
    pub docs: Vec<models::Doc>,
}

impl WorkspaceWithDocs {
    pub fn new(id: GristId, name: String, access: models::Access, docs: Vec<models::Doc>) -> WorkspaceWithDocs {
        WorkspaceWithDocs {
            id,
            name,
            access,
            docs,
        }
    }
}

