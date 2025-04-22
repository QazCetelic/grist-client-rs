use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateColumnsColumnsInner {
    /// Column identifier
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "fields")]
    pub fields: Box<models::UpdateColumnsColumnsInnerFields>,
}

impl UpdateColumnsColumnsInner {
    pub fn new(id: String, fields: models::UpdateColumnsColumnsInnerFields) -> UpdateColumnsColumnsInner {
        UpdateColumnsColumnsInner {
            id,
            fields: Box::new(fields),
        }
    }
}

