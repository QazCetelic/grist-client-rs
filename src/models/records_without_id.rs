use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecordsWithoutId {
    #[serde(rename = "records")]
    pub records: Vec<models::RecordsWithoutIdRecordsInner>,
}

impl RecordsWithoutId {
    pub fn new(records: Vec<models::RecordsWithoutIdRecordsInner>) -> RecordsWithoutId {
        RecordsWithoutId {
            records,
        }
    }
}

