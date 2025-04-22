use crate::models;
use serde::{Deserialize, Serialize};

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Access {
    #[serde(rename = "owners")]
    Owners,
    #[serde(rename = "editors")]
    Editors,
    #[serde(rename = "viewers")]
    Viewers,

}

impl std::fmt::Display for Access {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Owners => write!(f, "owners"),
            Self::Editors => write!(f, "editors"),
            Self::Viewers => write!(f, "viewers"),
        }
    }
}

impl Default for Access {
    fn default() -> Access {
        Self::Owners
    }
}

