use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DocsDocIdWebhooksPost200Response {
    #[serde(rename = "webhooks")]
    pub webhooks: Vec<models::WebhookId>,
}

impl DocsDocIdWebhooksPost200Response {
    pub fn new(webhooks: Vec<models::WebhookId>) -> DocsDocIdWebhooksPost200Response {
        DocsDocIdWebhooksPost200Response {
            webhooks,
        }
    }
}

