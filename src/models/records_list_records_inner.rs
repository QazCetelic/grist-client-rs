use crate::models;
use serde::{Deserialize, Serialize};
use crate::models::primitive_types::GristId;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecordsListRecordsInner {
    #[serde(rename = "id")]
    pub id: GristId,
    /// A JSON object mapping column names to [cell values](https://support.getgrist.com/code/modules/GristData/#cellvalue).
    #[serde(rename = "fields")]
    pub fields: serde_json::Map<String, serde_json::Value>,
}

impl RecordsListRecordsInner {
    pub fn new(id: GristId, fields: serde_json::Map<String, serde_json::Value>) -> RecordsListRecordsInner {
        RecordsListRecordsInner {
            id,
            fields,
        }
    }
}

