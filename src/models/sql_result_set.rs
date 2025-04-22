use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlResultSet {
    /// A copy of the SQL statement.
    #[serde(rename = "statement")]
    pub statement: String,
    #[serde(rename = "records")]
    pub records: Vec<models::SqlResultSetRecordsInner>,
}

impl SqlResultSet {
    pub fn new(statement: String, records: Vec<models::SqlResultSetRecordsInner>) -> SqlResultSet {
        SqlResultSet {
            statement,
            records,
        }
    }
}

