use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteActionsRequest {
    /// The number of the latest history actions to keep
    #[serde(rename = "keep")]
    pub keep: i32,
}

impl DeleteActionsRequest {
    pub fn new(keep: i32) -> DeleteActionsRequest {
        DeleteActionsRequest {
            keep,
        }
    }
}

