use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ColumnsWithoutFields {
    #[serde(rename = "columns")]
    pub columns: Vec<models::ColumnsWithoutFieldsColumnsInner>,
}

impl ColumnsWithoutFields {
    pub fn new(columns: Vec<models::ColumnsWithoutFieldsColumnsInner>) -> ColumnsWithoutFields {
        ColumnsWithoutFields {
            columns,
        }
    }
}

