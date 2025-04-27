use crate::models;
use serde::{Deserialize, Serialize};
use crate::models::primitive_types::GristId;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DocsDocIdSqlPostRequestArgsInner {
    Number(GristId),
    String(String),
}

impl Default for DocsDocIdSqlPostRequestArgsInner {
    fn default() -> Self {
        Self::Number(Default::default())
    }
}

