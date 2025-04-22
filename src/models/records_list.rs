use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecordsList {
    #[serde(rename = "records")]
    pub records: Vec<models::RecordsListRecordsInner>,
}

impl RecordsList {
    pub fn new(records: Vec<models::RecordsListRecordsInner>) -> RecordsList {
        RecordsList {
            records,
        }
    }
}

