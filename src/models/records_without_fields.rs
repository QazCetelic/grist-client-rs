use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecordsWithoutFields {
    #[serde(rename = "records")]
    pub records: Vec<models::RecordsWithoutFieldsRecordsInner>,
}

impl RecordsWithoutFields {
    pub fn new(records: Vec<models::RecordsWithoutFieldsRecordsInner>) -> RecordsWithoutFields {
        RecordsWithoutFields {
            records,
        }
    }
}

