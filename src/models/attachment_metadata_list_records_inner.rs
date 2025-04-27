use crate::models;
use serde::{Deserialize, Serialize};
use crate::models::primitive_types::GristId;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttachmentMetadataListRecordsInner {
    #[serde(rename = "id")]
    pub id: GristId,
    #[serde(rename = "fields")]
    pub fields: Box<models::AttachmentMetadata>,
}

impl AttachmentMetadataListRecordsInner {
    pub fn new(id: GristId, fields: models::AttachmentMetadata) -> AttachmentMetadataListRecordsInner {
        AttachmentMetadataListRecordsInner {
            id,
            fields: Box::new(fields),
        }
    }
}

