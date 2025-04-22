use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttachmentMetadata {
    #[serde(rename = "fileName", skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(rename = "fileSize", skip_serializing_if = "Option::is_none")]
    pub file_size: Option<usize>,
    #[serde(rename = "timeUploaded", skip_serializing_if = "Option::is_none")]
    pub time_uploaded: Option<String>,
}

impl AttachmentMetadata {
    pub fn new() -> AttachmentMetadata {
        AttachmentMetadata {
            file_name: None,
            file_size: None,
            time_uploaded: None,
        }
    }
}

