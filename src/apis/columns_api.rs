use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};


/// struct for typed errors of method [`add_columns`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddColumnsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_column`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteColumnError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_columns`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListColumnsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`modify_columns`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ModifyColumnsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`replace_columns`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReplaceColumnsError {
    UnknownValue(serde_json::Value),
}


pub async fn add_columns(configuration: &configuration::Configuration, doc_id: &str, table_id: &str, create_columns: models::CreateColumns) -> Result<models::ColumnsWithoutFields, Error<AddColumnsError>> {
    let uri_str = format!("{base_path}/docs/{docId}/tables/{tableId}/columns",
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
    req_builder = req_builder.json(&create_columns);

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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ColumnsWithoutFields`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ColumnsWithoutFields`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AddColumnsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn delete_column(configuration: &configuration::Configuration, doc_id: &str, table_id: &str, col_id: &str) -> Result<(), Error<DeleteColumnError>> {
    let uri_str = format!("{base_path}/docs/{docId}/tables/{tableId}/columns/{colId}",
        base_path = configuration.base_path,
        docId = crate::apis::urlencode(doc_id),
        tableId = crate::apis::urlencode(table_id),
        colId = crate::apis::urlencode(col_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteColumnError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn list_columns(configuration: &configuration::Configuration, doc_id: &str, table_id: &str, hidden: Option<bool>) -> Result<models::ColumnsList, Error<ListColumnsError>> {
    let uri_str = format!("{base_path}/docs/{docId}/tables/{tableId}/columns",
        base_path = configuration.base_path,
        docId = crate::apis::urlencode(doc_id),
        tableId = crate::apis::urlencode(table_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = hidden {
        req_builder = req_builder.query(&[("hidden", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ColumnsList`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ColumnsList`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ListColumnsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn modify_columns(configuration: &configuration::Configuration, doc_id: &str, table_id: &str, update_columns: models::UpdateColumns) -> Result<(), Error<ModifyColumnsError>> {
    let uri_str = format!("{base_path}/docs/{docId}/tables/{tableId}/columns",
        base_path = configuration.base_path,
        docId = crate::apis::urlencode(doc_id),
        tableId = crate::apis::urlencode(table_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&update_columns);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<ModifyColumnsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn replace_columns(configuration: &configuration::Configuration, doc_id: &str, table_id: &str, update_columns: models::UpdateColumns, noadd: Option<bool>, noupdate: Option<bool>, replaceall: Option<bool>) -> Result<(), Error<ReplaceColumnsError>> {
    let uri_str = format!("{base_path}/docs/{docId}/tables/{tableId}/columns",
        base_path = configuration.base_path,
        docId = crate::apis::urlencode(doc_id),
        tableId = crate::apis::urlencode(table_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref param_value) = noadd {
        req_builder = req_builder.query(&[("noadd", &param_value.to_string())]);
    }
    if let Some(ref param_value) = noupdate {
        req_builder = req_builder.query(&[("noupdate", &param_value.to_string())]);
    }
    if let Some(ref param_value) = replaceall {
        req_builder = req_builder.query(&[("replaceall", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&update_columns);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<ReplaceColumnsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

