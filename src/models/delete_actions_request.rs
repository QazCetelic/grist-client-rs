use crate::models;
use serde::{Deserialize, Serialize};
use crate::models::primitive_types::GristCount;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteActionsRequest {
    /// The number of the latest history actions to keep
    #[serde(rename = "keep")]
    pub keep: GristCount,
}

impl DeleteActionsRequest {
    pub fn new(keep: GristCount) -> DeleteActionsRequest {
        DeleteActionsRequest {
            keep,
        }
    }
}

