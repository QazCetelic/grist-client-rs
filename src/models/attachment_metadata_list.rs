use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttachmentMetadataList {
    #[serde(rename = "records")]
    pub records: Vec<models::AttachmentMetadataListRecordsInner>,
}

impl AttachmentMetadataList {
    pub fn new(records: Vec<models::AttachmentMetadataListRecordsInner>) -> AttachmentMetadataList {
        AttachmentMetadataList {
            records,
        }
    }
}

