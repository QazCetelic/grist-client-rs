use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ColumnsList {
    #[serde(rename = "columns", skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<models::ColumnsListColumnsInner>>,
}

impl ColumnsList {
    pub fn new() -> ColumnsList {
        ColumnsList {
            columns: None,
        }
    }
}

