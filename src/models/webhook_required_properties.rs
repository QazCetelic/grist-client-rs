use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookRequiredProperties {
    #[serde(rename = "size")]
    pub size: usize,
}

impl WebhookRequiredProperties {
    pub fn new(size: usize) -> WebhookRequiredProperties {
        WebhookRequiredProperties {
            size,
        }
    }
}

