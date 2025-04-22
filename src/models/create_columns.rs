use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateColumns {
    #[serde(rename = "columns")]
    pub columns: Vec<models::CreateColumnsColumnsInner>,
}

impl CreateColumns {
    pub fn new(columns: Vec<models::CreateColumnsColumnsInner>) -> CreateColumns {
        CreateColumns {
            columns,
        }
    }
}

