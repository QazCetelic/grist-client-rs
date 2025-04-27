use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DocsDocIdWebhooksPostRequest {
    #[serde(rename = "webhooks")]
    pub webhooks: Vec<models::DocsDocIdWebhooksPostRequestWebhooksInner>,
}

impl DocsDocIdWebhooksPostRequest {
    pub fn new(webhooks: Vec<models::DocsDocIdWebhooksPostRequestWebhooksInner>) -> DocsDocIdWebhooksPostRequest {
        DocsDocIdWebhooksPostRequest {
            webhooks,
        }
    }
}

