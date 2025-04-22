use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TablesWithoutFieldsTablesInner {
    #[serde(rename = "id")]
    pub id: String,
}

impl TablesWithoutFieldsTablesInner {
    pub fn new(id: String) -> TablesWithoutFieldsTablesInner {
        TablesWithoutFieldsTablesInner {
            id,
        }
    }
}

