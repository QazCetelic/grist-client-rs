use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecordsWithoutIdRecordsInner {
    /// A JSON object mapping column names to [cell values](https://support.getgrist.com/code/modules/GristData/#cellvalue).
    #[serde(rename = "fields")]
    pub fields: serde_json::Value,
}

impl RecordsWithoutIdRecordsInner {
    pub fn new(fields: serde_json::Value) -> RecordsWithoutIdRecordsInner {
        RecordsWithoutIdRecordsInner {
            fields,
        }
    }
}

