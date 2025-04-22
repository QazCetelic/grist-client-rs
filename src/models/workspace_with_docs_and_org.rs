use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceWithDocsAndOrg {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "name")]
    pub name: String,
    // This isn't actually there
    /*#[serde(rename = "access")]
    pub access: models::Access,*/
    #[serde(rename = "docs")]
    pub docs: Vec<models::Doc>,
    #[serde(rename = "org")]
    pub org: Box<models::Org>,
}

impl WorkspaceWithDocsAndOrg {
    pub fn new(id: i64, name: String, /*access: models::Access,*/ docs: Vec<models::Doc>, org: models::Org) -> WorkspaceWithDocsAndOrg {
        WorkspaceWithDocsAndOrg {
            id,
            name,
            // access,
            docs,
            org: Box::new(org),
        }
    }
}

