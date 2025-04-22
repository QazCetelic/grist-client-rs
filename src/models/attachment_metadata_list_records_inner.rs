use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttachmentMetadataListRecordsInner {
    #[serde(rename = "id")]
    pub id: u64,
    #[serde(rename = "fields")]
    pub fields: Box<models::AttachmentMetadata>,
}

impl AttachmentMetadataListRecordsInner {
    pub fn new(id: u64, fields: models::AttachmentMetadata) -> AttachmentMetadataListRecordsInner {
        AttachmentMetadataListRecordsInner {
            id,
            fields: Box::new(fields),
        }
    }
}

