use crate::models;
use serde::{Deserialize, Serialize};
use crate::models::primitive_types::GristDuration;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DocsDocIdSqlPostRequest {
    /// The SQL query to run. Must be a single SELECT statement, with no trailing semicolon. WITH clauses are permitted. All Grist documents are currently SQLite databases, and the SQL query is interpreted and run by SQLite, with various defensive measures. Statements that would modify the database are not supported.
    #[serde(rename = "sql")]
    pub sql: String,
    /// Parameters for the query.
    #[serde(rename = "args", skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<models::DocsDocIdSqlPostRequestArgsInner>>,
    /// Timeout after which operations on the document will be interrupted. Specified in milliseconds. Defaults to 1000 (1 second). This default is controlled by an optional environment variable read by the Grist app, GRIST_SQL_TIMEOUT_MSEC. The default cannot be exceeded, only reduced.
    #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
    pub timeout: Option<GristDuration>,
}

impl DocsDocIdSqlPostRequest {
    pub fn new(sql: String) -> DocsDocIdSqlPostRequest {
        DocsDocIdSqlPostRequest {
            sql,
            args: None,
            timeout: None,
        }
    }
}

