use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ModifyWorkspaceAccessRequest {
    #[serde(rename = "delta")]
    pub delta: Box<models::WorkspaceAccessWrite>,
}

impl ModifyWorkspaceAccessRequest {
    pub fn new(delta: models::WorkspaceAccessWrite) -> ModifyWorkspaceAccessRequest {
        ModifyWorkspaceAccessRequest {
            delta: Box::new(delta),
        }
    }
}

