use crate::models;
use serde::{Deserialize, Serialize};
use crate::models::primitive_types::GristId;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Org {
    #[serde(rename = "id")]
    pub id: GristId,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "domain", deserialize_with = "Option::deserialize")]
    pub domain: Option<String>,
    #[serde(rename = "owner")]
    pub owner: models::User,
    /// This isn't actually always provided by the API, unlike stated by the spec
    #[serde(rename = "access")]
    pub access: Option<models::Access>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

impl Org {
    pub fn new(id: GristId, name: String, domain: Option<String>, owner: models::User, access: models::Access, created_at: String, updated_at: String) -> Org {
        Org {
            id,
            name,
            domain,
            owner,
            access: Some(access),
            created_at,
            updated_at,
        }
    }
}

