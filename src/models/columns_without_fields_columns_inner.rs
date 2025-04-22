use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ColumnsWithoutFieldsColumnsInner {
    #[serde(rename = "id")]
    pub id: String,
}

impl ColumnsWithoutFieldsColumnsInner {
    pub fn new(id: String) -> ColumnsWithoutFieldsColumnsInner {
        ColumnsWithoutFieldsColumnsInner {
            id,
        }
    }
}

