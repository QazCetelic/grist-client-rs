use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookFields {
    #[serde(rename = "name", deserialize_with = "Option::deserialize")]
    pub name: Option<String>,
    #[serde(rename = "memo", deserialize_with = "Option::deserialize")]
    pub memo: Option<String>,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(rename = "eventTypes")]
    pub event_types: Vec<String>,
    #[serde(rename = "isReadyColumn", deserialize_with = "Option::deserialize")]
    pub is_ready_column: Option<String>,
    #[serde(rename = "tableId")]
    pub table_id: String,
    #[serde(rename = "unsubscribeKey")]
    pub unsubscribe_key: String,
}

impl WebhookFields {
    pub fn new(name: Option<String>, memo: Option<String>, url: String, enabled: bool, event_types: Vec<String>, is_ready_column: Option<String>, table_id: String, unsubscribe_key: String) -> WebhookFields {
        WebhookFields {
            name,
            memo,
            url,
            enabled,
            event_types,
            is_ready_column,
            table_id,
            unsubscribe_key,
        }
    }
}

