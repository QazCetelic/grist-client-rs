use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateTablesTablesInner {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "columns")]
    pub columns: Vec<models::CreateTablesTablesInnerColumnsInner>,
}

impl CreateTablesTablesInner {
    pub fn new(columns: Vec<models::CreateTablesTablesInnerColumnsInner>) -> CreateTablesTablesInner {
        CreateTablesTablesInner {
            id: None,
            columns,
        }
    }
}

