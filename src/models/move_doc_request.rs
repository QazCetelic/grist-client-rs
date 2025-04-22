use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MoveDocRequest {
    #[serde(rename = "workspace")]
    pub workspace: i32,
}

impl MoveDocRequest {
    pub fn new(workspace: i32) -> MoveDocRequest {
        MoveDocRequest {
            workspace,
        }
    }
}

