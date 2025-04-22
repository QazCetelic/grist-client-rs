use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TablesListTablesInner {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "fields")]
    pub fields: serde_json::Value,
}

impl TablesListTablesInner {
    pub fn new(id: String, fields: serde_json::Value) -> TablesListTablesInner {
        TablesListTablesInner {
            id,
            fields,
        }
    }
}

