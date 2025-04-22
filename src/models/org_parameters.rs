use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrgParameters {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl OrgParameters {
    pub fn new() -> OrgParameters {
        OrgParameters {
            name: None,
        }
    }
}

