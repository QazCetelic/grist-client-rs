use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TablesWithoutFields {
    #[serde(rename = "tables")]
    pub tables: Vec<models::TablesWithoutFieldsTablesInner>,
}

impl TablesWithoutFields {
    pub fn new(tables: Vec<models::TablesWithoutFieldsTablesInner>) -> TablesWithoutFields {
        TablesWithoutFields {
            tables,
        }
    }
}

