use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DocParameters {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "isPinned", skip_serializing_if = "Option::is_none")]
    pub is_pinned: Option<bool>,
}

impl DocParameters {
    pub fn new() -> DocParameters {
        DocParameters {
            name: None,
            is_pinned: None,
        }
    }
}

