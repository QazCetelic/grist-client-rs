use crate::models;
use serde::{Deserialize, Serialize};
use crate::models::primitive_types::GristId;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceWithDocsAndDomain {
    #[serde(rename = "id")]
    pub id: GristId,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "access")]
    pub access: models::Access,
    #[serde(rename = "docs")]
    pub docs: Vec<models::Doc>,
    #[serde(rename = "orgDomain", skip_serializing_if = "Option::is_none")]
    pub org_domain: Option<String>,
}

impl WorkspaceWithDocsAndDomain {
    pub fn new(id: GristId, name: String, access: models::Access, docs: Vec<models::Doc>) -> WorkspaceWithDocsAndDomain {
        WorkspaceWithDocsAndDomain {
            id,
            name,
            access,
            docs,
            org_domain: None,
        }
    }
}

