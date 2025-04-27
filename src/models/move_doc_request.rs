use crate::models;
use serde::{Deserialize, Serialize};
use crate::models::primitive_types::GristId;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MoveDocRequest {
    #[serde(rename = "workspace")]
    pub workspace: GristId,
}

impl MoveDocRequest {
    pub fn new(workspace: GristId) -> MoveDocRequest {
        MoveDocRequest {
            workspace,
        }
    }
}

