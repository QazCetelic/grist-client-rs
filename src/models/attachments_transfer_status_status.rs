use crate::models;
use serde::{Deserialize, Serialize};
use crate::models::primitive_types::GristCount;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttachmentsTransferStatusStatus {
    /// Remaining transfers be performed
    #[serde(rename = "pendingTransferCount", skip_serializing_if = "Option::is_none")]
    pub pending_transfer_count: Option<GristCount>,
    /// Are files actively being transferred?
    #[serde(rename = "isRunning", skip_serializing_if = "Option::is_none")]
    pub is_running: Option<bool>,
}

impl AttachmentsTransferStatusStatus {
    pub fn new() -> AttachmentsTransferStatusStatus {
        AttachmentsTransferStatusStatus {
            pending_transfer_count: None,
            is_running: None,
        }
    }
}

