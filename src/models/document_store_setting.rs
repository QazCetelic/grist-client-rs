use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DocumentStoreSetting {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
}

impl DocumentStoreSetting {
    pub fn new() -> DocumentStoreSetting {
        DocumentStoreSetting {
            r#type: None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "external")]
    External,
    #[serde(rename = "internal")]
    Internal,
}

impl Default for Type {
    fn default() -> Type {
        Self::External
    }
}

