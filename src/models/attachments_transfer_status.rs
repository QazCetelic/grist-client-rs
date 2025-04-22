use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttachmentsTransferStatus {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Box<models::AttachmentsTransferStatusStatus>>,
    #[serde(rename = "locationSummary", skip_serializing_if = "Option::is_none")]
    pub location_summary: Option<models::DocumentAttachmentsLocation>,
}

impl AttachmentsTransferStatus {
    pub fn new() -> AttachmentsTransferStatus {
        AttachmentsTransferStatus {
            status: None,
            location_summary: None,
        }
    }
}

