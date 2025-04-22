use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TableSchemaResult {
    /// The ID (technical name) of the table
    #[serde(rename = "name")]
    pub name: String,
    /// The human readable name of the table
    #[serde(rename = "title")]
    pub title: String,
    /// The URL to download the CSV
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "format", skip_serializing_if = "Option::is_none")]
    pub format: Option<Format>,
    #[serde(rename = "mediatype", skip_serializing_if = "Option::is_none")]
    pub mediatype: Option<Mediatype>,
    #[serde(rename = "encoding", skip_serializing_if = "Option::is_none")]
    pub encoding: Option<Encoding>,
    #[serde(rename = "dialect", skip_serializing_if = "Option::is_none")]
    pub dialect: Option<serde_json::Value>,
    #[serde(rename = "schema")]
    pub schema: serde_json::Value,
}

impl TableSchemaResult {
    pub fn new(name: String, title: String, schema: serde_json::Value) -> TableSchemaResult {
        TableSchemaResult {
            name,
            title,
            path: None,
            format: None,
            mediatype: None,
            encoding: None,
            dialect: None,
            schema,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Format {
    #[serde(rename = "csv")]
    Csv,
}

impl Default for Format {
    fn default() -> Format {
        Self::Csv
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Mediatype {
    #[serde(rename = "text/csv")]
    TextSlashCsv,
}

impl Default for Mediatype {
    fn default() -> Mediatype {
        Self::TextSlashCsv
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Encoding {
    #[serde(rename = "utf-8")]
    Utf8,
}

impl Default for Encoding {
    fn default() -> Encoding {
        Self::Utf8
    }
}

