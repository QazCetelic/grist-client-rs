use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ModifyOrgAccessRequest {
    #[serde(rename = "delta")]
    pub delta: Box<models::OrgAccessWrite>,
}

impl ModifyOrgAccessRequest {
    pub fn new(delta: models::OrgAccessWrite) -> ModifyOrgAccessRequest {
        ModifyOrgAccessRequest {
            delta: Box::new(delta),
        }
    }
}

