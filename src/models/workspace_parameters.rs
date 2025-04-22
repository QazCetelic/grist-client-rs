use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceParameters {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl WorkspaceParameters {
    pub fn new() -> WorkspaceParameters {
        WorkspaceParameters {
            name: None,
        }
    }
}

