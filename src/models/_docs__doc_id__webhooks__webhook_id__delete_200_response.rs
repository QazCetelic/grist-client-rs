use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DocsDocIdWebhooksWebhookIdDelete200Response {
    #[serde(rename = "success")]
    pub success: bool,
}

impl DocsDocIdWebhooksWebhookIdDelete200Response {
    pub fn new(success: bool) -> DocsDocIdWebhooksWebhookIdDelete200Response {
        DocsDocIdWebhooksWebhookIdDelete200Response {
            success,
        }
    }
}

