use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DocsDocIdWebhooksGet200Response {
    #[serde(rename = "webhooks")]
    pub webhooks: Vec<models::Webhook>,
}

impl DocsDocIdWebhooksGet200Response {
    pub fn new(webhooks: Vec<models::Webhook>) -> DocsDocIdWebhooksGet200Response {
        DocsDocIdWebhooksGet200Response {
            webhooks,
        }
    }
}

