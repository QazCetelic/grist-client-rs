use crate::models;
use serde::{Deserialize, Serialize};
use crate::models::primitive_types::GristSize;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookRequiredProperties {
    #[serde(rename = "size")]
    pub size: GristSize,
}

impl WebhookRequiredProperties {
    pub fn new(size: GristSize) -> WebhookRequiredProperties {
        WebhookRequiredProperties {
            size,
        }
    }
}

