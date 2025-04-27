use crate::models;
use serde::{Deserialize, Serialize};
use crate::models::primitive_types::GristId;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Data {
    #[serde(rename = "id")]
    pub id: Vec<GristId>,
}

impl Data {
    pub fn new(id: Vec<GristId>) -> Data {
        Data {
            id,
        }
    }
}

