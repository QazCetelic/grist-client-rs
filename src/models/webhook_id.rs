use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookId {
    /// Webhook identifier
    #[serde(rename = "id")]
    pub id: String,
}

impl WebhookId {
    pub fn new(id: String) -> WebhookId {
        WebhookId {
            id,
        }
    }
}

