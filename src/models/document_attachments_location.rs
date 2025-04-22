use crate::models;
use serde::{Deserialize, Serialize};

/// DocumentAttachmentsLocation : The current location of attachment files in the document
/// The current location of attachment files in the document
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DocumentAttachmentsLocation {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "internal")]
    Internal,
    #[serde(rename = "mixed")]
    Mixed,
    #[serde(rename = "external")]
    External,

}

impl std::fmt::Display for DocumentAttachmentsLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "none"),
            Self::Internal => write!(f, "internal"),
            Self::Mixed => write!(f, "mixed"),
            Self::External => write!(f, "external"),
        }
    }
}

impl Default for DocumentAttachmentsLocation {
    fn default() -> DocumentAttachmentsLocation {
        Self::None
    }
}

