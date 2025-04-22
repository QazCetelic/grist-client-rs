use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookPartialFields {
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "memo", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub memo: Option<Option<String>>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "eventTypes", skip_serializing_if = "Option::is_none")]
    pub event_types: Option<Vec<String>>,
    #[serde(rename = "isReadyColumn", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub is_ready_column: Option<Option<String>>,
    #[serde(rename = "tableId", skip_serializing_if = "Option::is_none")]
    pub table_id: Option<String>,
}

impl WebhookPartialFields {
    pub fn new() -> WebhookPartialFields {
        WebhookPartialFields {
            name: None,
            memo: None,
            url: None,
            enabled: None,
            event_types: None,
            is_ready_column: None,
            table_id: None,
        }
    }
}

