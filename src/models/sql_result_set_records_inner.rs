use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlResultSetRecordsInner {
    #[serde(rename = "fields")]
    pub fields: serde_json::Value,
}

impl SqlResultSetRecordsInner {
    pub fn new(fields: serde_json::Value) -> SqlResultSetRecordsInner {
        SqlResultSetRecordsInner {
            fields,
        }
    }
}

