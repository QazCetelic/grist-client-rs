use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateTables {
    #[serde(rename = "tables")]
    pub tables: Vec<models::CreateTablesTablesInner>,
}

impl CreateTables {
    pub fn new(tables: Vec<models::CreateTablesTablesInner>) -> CreateTables {
        CreateTables {
            tables,
        }
    }
}

