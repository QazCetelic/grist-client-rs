use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TablesList {
    #[serde(rename = "tables")]
    pub tables: Vec<models::TablesListTablesInner>,
}

impl TablesList {
    pub fn new(tables: Vec<models::TablesListTablesInner>) -> TablesList {
        TablesList {
            tables,
        }
    }
}

