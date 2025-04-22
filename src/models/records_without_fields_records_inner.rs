use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecordsWithoutFieldsRecordsInner {
    #[serde(rename = "id")]
    pub id: usize,
}

impl RecordsWithoutFieldsRecordsInner {
    pub fn new(id: usize) -> RecordsWithoutFieldsRecordsInner {
        RecordsWithoutFieldsRecordsInner {
            id,
        }
    }
}

