use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};


/// struct for typed errors of method [`add_tables`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddTablesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_tables`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListTablesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`modify_tables`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ModifyTablesError {
    UnknownValue(serde_json::Value),
}


pub async fn add_tables(configuration: &configuration::Configuration, doc_id: &str, create_tables: models::CreateTables) -> Result<models::TablesWithoutFields, Error<AddTablesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_doc_id = doc_id;
    let p_create_tables = create_tables;

    let uri_str = format!("{}/docs/{docId}/tables", configuration.base_path, docId=crate::apis::urlencode(p_doc_id));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_create_tables);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::TablesWithoutFields`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::TablesWithoutFields`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AddTablesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn list_tables(configuration: &configuration::Configuration, doc_id: &str) -> Result<models::TablesList, Error<ListTablesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_doc_id = doc_id;

    let uri_str = format!("{}/docs/{docId}/tables", configuration.base_path, docId=crate::apis::urlencode(p_doc_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::TablesList`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::TablesList`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ListTablesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn modify_tables(configuration: &configuration::Configuration, doc_id: &str, tables_list: models::TablesList) -> Result<(), Error<ModifyTablesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_doc_id = doc_id;
    let p_tables_list = tables_list;

    let uri_str = format!("{}/docs/{docId}/tables", configuration.base_path, docId=crate::apis::urlencode(p_doc_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_tables_list);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<ModifyTablesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

