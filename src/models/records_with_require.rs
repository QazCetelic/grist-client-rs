use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecordsWithRequire {
    #[serde(rename = "records")]
    pub records: Vec<models::RecordsWithRequireRecordsInner>,
}

impl RecordsWithRequire {
    pub fn new(records: Vec<models::RecordsWithRequireRecordsInner>) -> RecordsWithRequire {
        RecordsWithRequire {
            records,
        }
    }
}

