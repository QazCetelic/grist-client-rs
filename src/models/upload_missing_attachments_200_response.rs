use crate::models;
use serde::{Deserialize, Serialize};
use crate::models::primitive_types::GristCount;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UploadMissingAttachments200Response {
    /// Total files added to external storage.
    #[serde(rename = "added", skip_serializing_if = "Option::is_none")]
    pub added: Option<GristCount>,
    /// Total files that errored when attempting to process them.
    #[serde(rename = "errored", skip_serializing_if = "Option::is_none")]
    pub errored: Option<GristCount>,
    /// Total files that aren't needed, or don't match an existing attachment.
    #[serde(rename = "unused", skip_serializing_if = "Option::is_none")]
    pub unused: Option<GristCount>,
}

impl UploadMissingAttachments200Response {
    pub fn new() -> UploadMissingAttachments200Response {
        UploadMissingAttachments200Response {
            added: None,
            errored: None,
            unused: None,
        }
    }
}

