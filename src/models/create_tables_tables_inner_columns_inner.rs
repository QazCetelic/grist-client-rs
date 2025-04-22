use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateTablesTablesInnerColumnsInner {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "fields", skip_serializing_if = "Option::is_none")]
    pub fields: Option<serde_json::Value>,
}

impl CreateTablesTablesInnerColumnsInner {
    pub fn new() -> CreateTablesTablesInnerColumnsInner {
        CreateTablesTablesInnerColumnsInner {
            id: None,
            fields: None,
        }
    }
}

