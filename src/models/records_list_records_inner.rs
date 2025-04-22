use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecordsListRecordsInner {
    #[serde(rename = "id")]
    pub id: u64,
    /// A JSON object mapping column names to [cell values](https://support.getgrist.com/code/modules/GristData/#cellvalue).
    #[serde(rename = "fields")]
    pub fields: serde_json::Map<String, serde_json::Value>,
}

impl RecordsListRecordsInner {
    pub fn new(id: u64, fields: serde_json::Map<String, serde_json::Value>) -> RecordsListRecordsInner {
        RecordsListRecordsInner {
            id,
            fields,
        }
    }
}

