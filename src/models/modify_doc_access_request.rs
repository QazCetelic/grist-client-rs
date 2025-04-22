use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ModifyDocAccessRequest {
    #[serde(rename = "delta")]
    pub delta: Box<models::WorkspaceAccessWrite>,
}

impl ModifyDocAccessRequest {
    pub fn new(delta: models::WorkspaceAccessWrite) -> ModifyDocAccessRequest {
        ModifyDocAccessRequest {
            delta: Box::new(delta),
        }
    }
}

