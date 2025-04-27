use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DocsDocIdWebhooksPostRequestWebhooksInner {
    #[serde(rename = "fields")]
    pub fields: Box<models::WebhookPartialFields>,
}

impl DocsDocIdWebhooksPostRequestWebhooksInner {
    pub fn new(fields: models::WebhookPartialFields) -> DocsDocIdWebhooksPostRequestWebhooksInner {
        DocsDocIdWebhooksPostRequestWebhooksInner {
            fields: Box::new(fields),
        }
    }
}

