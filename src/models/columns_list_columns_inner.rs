use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ColumnsListColumnsInner {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "fields", skip_serializing_if = "Option::is_none")]
    pub fields: Option<Box<models::GetFields>>,
}

impl ColumnsListColumnsInner {
    pub fn new() -> ColumnsListColumnsInner {
        ColumnsListColumnsInner {
            id: None,
            fields: None,
        }
    }
}

