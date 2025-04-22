use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DescribeOrgOrgIdParameter {
    Integer(i32),
    String(String),
}

impl Default for DescribeOrgOrgIdParameter {
    fn default() -> Self {
        Self::Integer(Default::default())
    }
}

