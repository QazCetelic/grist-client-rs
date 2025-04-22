use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookProperties {
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<usize>,
    #[serde(rename = "attempts", skip_serializing_if = "Option::is_none")]
    pub attempts: Option<usize>,
    #[serde(rename = "errorMessage", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<Option<String>>,
    #[serde(rename = "httpStatus", skip_serializing_if = "Option::is_none")]
    pub http_status: Option<usize>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl WebhookProperties {
    pub fn new() -> WebhookProperties {
        WebhookProperties {
            size: None,
            attempts: None,
            error_message: None,
            http_status: None,
            status: None,
        }
    }
}

