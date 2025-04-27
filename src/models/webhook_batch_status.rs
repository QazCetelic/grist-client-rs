use crate::models;
use serde::{Deserialize, Serialize};
use crate::models::primitive_types::{GristSize, GristHttpCode, GristCount};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookBatchStatus {
    #[serde(rename = "size")]
    pub size: GristSize,
    #[serde(rename = "attempts")]
    pub attempts: GristCount,
    #[serde(rename = "errorMessage", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<Option<String>>,
    #[serde(rename = "httpStatus", skip_serializing_if = "Option::is_none")]
    pub http_status: Option<GristHttpCode>,
    #[serde(rename = "status")]
    pub status: String,
}

impl WebhookBatchStatus {
    pub fn new(size: GristSize, attempts: GristCount, status: String) -> WebhookBatchStatus {
        WebhookBatchStatus {
            size,
            attempts,
            error_message: None,
            http_status: None,
            status,
        }
    }
}

