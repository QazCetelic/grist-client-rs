use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};


/// struct for typed errors of method [`add_rows`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddRowsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_rows`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteRowsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_table_data`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTableDataError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`modify_rows`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ModifyRowsError {
    UnknownValue(serde_json::Value),
}


/// Deprecated in favor of `records` endpoints. We have no immediate plans to remove these endpoints, but consider `records` a better starting point for new projects.
pub async fn add_rows(configuration: &configuration::Configuration, doc_id: &str, table_id: &str, request_body: std::collections::HashMap<String, Vec<serde_json::Value>>, noparse: Option<bool>) -> Result<Vec<i32>, Error<AddRowsError>> {
    let uri_str = format!("{base_path}/docs/{docId}/tables/{tableId}/data",
        base_path = configuration.base_path,
        docId = crate::apis::urlencode(doc_id),
        tableId = crate::apis::urlencode(table_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref param_value) = noparse {
        req_builder = req_builder.query(&[("noparse", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&request_body);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `Vec&lt;i32&gt;`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `Vec&lt;i32&gt;`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AddRowsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn delete_rows(configuration: &configuration::Configuration, doc_id: &str, table_id: &str, request_body: Vec<i32>) -> Result<(), Error<DeleteRowsError>> {
    let uri_str = format!("{base_path}/docs/{docId}/tables/{tableId}/data/delete",
        base_path = configuration.base_path,
        docId = crate::apis::urlencode(doc_id),
        tableId = crate::apis::urlencode(table_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&request_body);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteRowsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Deprecated in favor of `records` endpoints. We have no immediate plans to remove these endpoints, but consider `records` a better starting point for new projects.
pub async fn get_table_data(configuration: &configuration::Configuration, doc_id: &str, table_id: &str, filter: Option<&str>, sort: Option<&str>, limit: Option<f64>, x_sort: Option<&str>, x_limit: Option<f64>) -> Result<models::Data, Error<GetTableDataError>> {
    let uri_str = format!("{base_path}/docs/{docId}/tables/{tableId}/data", base_path = configuration.base_path, docId=crate::apis::urlencode(doc_id), tableId=crate::apis::urlencode(table_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = filter {
        req_builder = req_builder.query(&[("filter", &param_value.to_string())]);
    }
    if let Some(ref param_value) = sort {
        req_builder = req_builder.query(&[("sort", &param_value.to_string())]);
    }
    if let Some(ref param_value) = limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = sort {
        req_builder = req_builder.header("X-Sort", param_value.to_string());
    }
    if let Some(param_value) = limit {
        req_builder = req_builder.header("X-Limit", param_value.to_string());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Data`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Data`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetTableDataError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Deprecated in favor of `records` endpoints. We have no immediate plans to remove these endpoints, but consider `records` a better starting point for new projects.
pub async fn modify_rows(configuration: &configuration::Configuration, doc_id: &str, table_id: &str, data: models::Data, noparse: Option<bool>) -> Result<Vec<i32>, Error<ModifyRowsError>> {
    let uri_str = format!("{base_path}/docs/{docId}/tables/{tableId}/data", base_path = configuration.base_path, docId=crate::apis::urlencode(doc_id), tableId=crate::apis::urlencode(table_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref param_value) = noparse {
        req_builder = req_builder.query(&[("noparse", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&data);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `Vec&lt;i32&gt;`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `Vec&lt;i32&gt;`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ModifyRowsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

