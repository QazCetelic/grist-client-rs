use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DocsDocIdSqlPostRequestArgsInner {
    Number(usize),
    String(String),
}

impl Default for DocsDocIdSqlPostRequestArgsInner {
    fn default() -> Self {
        Self::Number(Default::default())
    }
}

