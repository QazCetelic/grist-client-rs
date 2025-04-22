use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateColumns {
    #[serde(rename = "columns")]
    pub columns: Vec<models::UpdateColumnsColumnsInner>,
}

impl UpdateColumns {
    pub fn new(columns: Vec<models::UpdateColumnsColumnsInner>) -> UpdateColumns {
        UpdateColumns {
            columns,
        }
    }
}

