use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SetDocumentAttachmentStore200Response {
    #[serde(rename = "store", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub store: Option<Option<String>>,
}

impl SetDocumentAttachmentStore200Response {
    pub fn new() -> SetDocumentAttachmentStore200Response {
        SetDocumentAttachmentStore200Response {
            store: None,
        }
    }
}

