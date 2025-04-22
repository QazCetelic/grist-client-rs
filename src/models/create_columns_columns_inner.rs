use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateColumnsColumnsInner {
    /// Column identifier
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "fields", skip_serializing_if = "Option::is_none")]
    pub fields: Option<Box<models::CreateFields>>,
}

impl CreateColumnsColumnsInner {
    pub fn new() -> CreateColumnsColumnsInner {
        CreateColumnsColumnsInner {
            id: None,
            fields: None,
        }
    }
}

