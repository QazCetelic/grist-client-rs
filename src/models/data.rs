use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Data {
    #[serde(rename = "id")]
    pub id: Vec<i32>,
}

impl Data {
    pub fn new(id: Vec<i32>) -> Data {
        Data {
            id,
        }
    }
}

