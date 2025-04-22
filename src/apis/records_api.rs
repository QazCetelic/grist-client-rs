use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};


/// struct for typed errors of method [`add_records`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddRecordsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_records`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListRecordsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`modify_records`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ModifyRecordsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`replace_records`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReplaceRecordsError {
    UnknownValue(serde_json::Value),
}


pub async fn add_records(configuration: &configuration::Configuration, doc_id: &str, table_id: &str, records_without_id: models::RecordsWithoutId, noparse: Option<bool>) -> Result<models::RecordsWithoutFields, Error<AddRecordsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_doc_id = doc_id;
    let p_table_id = table_id;
    let p_records_without_id = records_without_id;
    let p_noparse = noparse;

    let uri_str = format!("{}/docs/{docId}/tables/{tableId}/records", configuration.base_path, docId=crate::apis::urlencode(p_doc_id), tableId=crate::apis::urlencode(p_table_id));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref param_value) = p_noparse {
        req_builder = req_builder.query(&[("noparse", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_records_without_id);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::RecordsWithoutFields`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::RecordsWithoutFields`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AddRecordsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn list_records(configuration: &configuration::Configuration, doc_id: &str, table_id: &str, filter: Option<&str>, sort: Option<&str>, limit: Option<f64>, x_sort: Option<&str>, x_limit: Option<f64>, hidden: Option<bool>) -> Result<models::RecordsList, Error<ListRecordsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_doc_id = doc_id;
    let p_table_id = table_id;
    let p_filter = filter;
    let p_sort = sort;
    let p_limit = limit;
    let p_x_sort = x_sort;
    let p_x_limit = x_limit;
    let p_hidden = hidden;

    let uri_str = format!("{}/docs/{docId}/tables/{tableId}/records", configuration.base_path, docId=crate::apis::urlencode(p_doc_id), tableId=crate::apis::urlencode(p_table_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_filter {
        req_builder = req_builder.query(&[("filter", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_sort {
        req_builder = req_builder.query(&[("sort", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_hidden {
        req_builder = req_builder.query(&[("hidden", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_x_sort {
        req_builder = req_builder.header("X-Sort", param_value.to_string());
    }
    if let Some(param_value) = p_x_limit {
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::RecordsList`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::RecordsList`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ListRecordsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn modify_records(configuration: &configuration::Configuration, doc_id: &str, table_id: &str, records_list: models::RecordsList, noparse: Option<bool>) -> Result<(), Error<ModifyRecordsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_doc_id = doc_id;
    let p_table_id = table_id;
    let p_records_list = records_list;
    let p_noparse = noparse;

    let uri_str = format!("{}/docs/{docId}/tables/{tableId}/records", configuration.base_path, docId=crate::apis::urlencode(p_doc_id), tableId=crate::apis::urlencode(p_table_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref param_value) = p_noparse {
        req_builder = req_builder.query(&[("noparse", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_records_list);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<ModifyRecordsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn replace_records(configuration: &configuration::Configuration, doc_id: &str, table_id: &str, records_with_require: models::RecordsWithRequire, noparse: Option<bool>, onmany: Option<&str>, noadd: Option<bool>, noupdate: Option<bool>, allow_empty_require: Option<bool>) -> Result<(), Error<ReplaceRecordsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_doc_id = doc_id;
    let p_table_id = table_id;
    let p_records_with_require = records_with_require;
    let p_noparse = noparse;
    let p_onmany = onmany;
    let p_noadd = noadd;
    let p_noupdate = noupdate;
    let p_allow_empty_require = allow_empty_require;

    let uri_str = format!("{}/docs/{docId}/tables/{tableId}/records", configuration.base_path, docId=crate::apis::urlencode(p_doc_id), tableId=crate::apis::urlencode(p_table_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref param_value) = p_noparse {
        req_builder = req_builder.query(&[("noparse", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_onmany {
        req_builder = req_builder.query(&[("onmany", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_noadd {
        req_builder = req_builder.query(&[("noadd", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_noupdate {
        req_builder = req_builder.query(&[("noupdate", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_allow_empty_require {
        req_builder = req_builder.query(&[("allow_empty_require", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_records_with_require);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<ReplaceRecordsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

