use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Webhook {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[serde(rename = "fields", skip_serializing_if = "Option::is_none")]
    pub fields: Option<Box<models::WebhookFields>>,
    #[serde(rename = "usage", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub usage: Option<Option<Box<models::WebhookUsage>>>,
}

impl Webhook {
    pub fn new() -> Webhook {
        Webhook {
            id: None,
            fields: None,
            usage: None,
        }
    }
}

