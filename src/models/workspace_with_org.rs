use crate::models;
use serde::{Deserialize, Serialize};
use crate::models::primitive_types::GristId;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceWithOrg {
    #[serde(rename = "id")]
    pub id: GristId,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "access")]
    pub access: models::Access,
    #[serde(rename = "org")]
    pub org: Box<models::Org>,
}

impl WorkspaceWithOrg {
    pub fn new(id: GristId, name: String, access: models::Access, org: models::Org) -> WorkspaceWithOrg {
        WorkspaceWithOrg {
            id,
            name,
            access,
            org: Box::new(org),
        }
    }
}

