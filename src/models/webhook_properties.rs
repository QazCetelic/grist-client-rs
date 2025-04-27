use crate::models;
use serde::{Deserialize, Serialize};
use crate::models::primitive_types::{GristCount, GristHttpCode, GristSize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookProperties {
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<GristSize>,
    #[serde(rename = "attempts", skip_serializing_if = "Option::is_none")]
    pub attempts: Option<GristCount>,
    #[serde(rename = "errorMessage", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<Option<String>>,
    #[serde(rename = "httpStatus", skip_serializing_if = "Option::is_none")]
    pub http_status: Option<GristHttpCode>,
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

