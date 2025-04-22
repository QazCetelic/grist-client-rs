use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookUsage {
    #[serde(rename = "numWaiting")]
    pub num_waiting: i32,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "updatedTime", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<Option<usize>>,
    #[serde(rename = "lastSuccessTime", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_success_time: Option<Option<usize>>,
    #[serde(rename = "lastFailureTime", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_failure_time: Option<Option<usize>>,
    #[serde(rename = "lastErrorMessage", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_error_message: Option<Option<String>>,
    #[serde(rename = "lastHttpStatus", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_http_status: Option<Option<usize>>,
    #[serde(rename = "lastEventBatch", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_event_batch: Option<Option<Box<models::WebhookBatchStatus>>>,
}

impl WebhookUsage {
    pub fn new(num_waiting: i32, status: String) -> WebhookUsage {
        WebhookUsage {
            num_waiting,
            status,
            updated_time: None,
            last_success_time: None,
            last_failure_time: None,
            last_error_message: None,
            last_http_status: None,
            last_event_batch: None,
        }
    }
}

