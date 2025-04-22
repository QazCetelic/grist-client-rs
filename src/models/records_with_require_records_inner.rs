use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecordsWithRequireRecordsInner {
    /// keys are column identifiers, and values are [cell values](https://support.getgrist.com/code/modules/GristData/#cellvalue) we want to have in those columns (either by matching with an existing record, or creating a new record) 
    #[serde(rename = "require")]
    pub require: serde_json::Value,
    /// keys are column identifiers, and values are [cell values](https://support.getgrist.com/code/modules/GristData/#cellvalue) to place in those columns (either overwriting values in an existing record, or in a new record) 
    #[serde(rename = "fields", skip_serializing_if = "Option::is_none")]
    pub fields: Option<serde_json::Value>,
}

impl RecordsWithRequireRecordsInner {
    pub fn new(require: serde_json::Value) -> RecordsWithRequireRecordsInner {
        RecordsWithRequireRecordsInner {
            require,
            fields: None,
        }
    }
}

