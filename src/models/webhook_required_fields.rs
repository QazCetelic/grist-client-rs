use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookRequiredFields {
    #[serde(rename = "unsubscribeKey")]
    pub unsubscribe_key: String,
}

impl WebhookRequiredFields {
    pub fn new(unsubscribe_key: String) -> WebhookRequiredFields {
        WebhookRequiredFields {
            unsubscribe_key,
        }
    }
}

