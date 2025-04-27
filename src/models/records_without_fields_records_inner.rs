use crate::models;
use serde::{Deserialize, Serialize};
use crate::models::primitive_types::GristId;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecordsWithoutFieldsRecordsInner {
    #[serde(rename = "id")]
    pub id: GristId,
}

impl RecordsWithoutFieldsRecordsInner {
    pub fn new(id: GristId) -> RecordsWithoutFieldsRecordsInner {
        RecordsWithoutFieldsRecordsInner {
            id,
        }
    }
}

