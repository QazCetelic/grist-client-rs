use crate::models;
use serde::{Deserialize, Serialize};
use crate::models::primitive_types::GristId;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DescribeOrgOrgIdParameter {
    Integer(GristId),
    String(String),
}

impl Default for DescribeOrgOrgIdParameter {
    fn default() -> Self {
        Self::Integer(Default::default())
    }
}

