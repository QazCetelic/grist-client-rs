use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};


/// struct for typed errors of method [`create_doc`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateDocError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_actions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteActionsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_doc`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteDocError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`describe_doc`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DescribeDocError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`download_doc`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DownloadDocError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`download_doc_csv`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DownloadDocCsvError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`download_doc_xlsx`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DownloadDocXlsxError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`download_table_schema`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DownloadTableSchemaError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`force_reload`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ForceReloadError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_doc_access`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListDocAccessError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`modify_doc`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ModifyDocError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`modify_doc_access`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ModifyDocAccessError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`move_doc`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MoveDocError {
    UnknownValue(serde_json::Value),
}


pub async fn create_doc(configuration: &configuration::Configuration, workspace_id: i32, doc_parameters: models::DocParameters) -> Result<String, Error<CreateDocError>> {
    let uri_str = format!("{config}/workspaces/{workspaceId}/docs",
        config = configuration.base_path,
        workspaceId = workspace_id
    );
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&doc_parameters);

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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `String`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `String`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateDocError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn delete_actions(configuration: &configuration::Configuration, doc_id: &str, delete_actions_request: Option<models::DeleteActionsRequest>) -> Result<(), Error<DeleteActionsError>> {
    let uri_str = format!("{config}/docs/{docId}/states/remove",
        config = configuration.base_path,
        docId = crate::apis::urlencode(doc_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&delete_actions_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteActionsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn delete_doc(configuration: &configuration::Configuration, doc_id: &str) -> Result<(), Error<DeleteDocError>> {
    let uri_str = format!("{config}/docs/{docId}",
        config = configuration.base_path,
        docId = crate::apis::urlencode(doc_id)
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
        let entity: Option<DeleteDocError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn describe_doc(configuration: &configuration::Configuration, doc_id: &str) -> Result<models::DocWithWorkspace, Error<DescribeDocError>> {
    let uri_str = format!("{config}/docs/{docId}",
        config = configuration.base_path,
        docId = crate::apis::urlencode(doc_id)
    );
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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DocWithWorkspace`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DocWithWorkspace`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DescribeDocError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn download_doc(configuration: &configuration::Configuration, doc_id: &str, nohistory: Option<bool>, template: Option<bool>) -> Result<reqwest::Response, Error<DownloadDocError>> {
    let uri_str = format!("{config}/docs/{docId}/download",
        config = configuration.base_path,
        docId = crate::apis::urlencode(doc_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = nohistory {
        req_builder = req_builder.query(&[("nohistory", &param_value.to_string())]);
    }
    if let Some(ref param_value) = template {
        req_builder = req_builder.query(&[("template", &param_value.to_string())]);
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

    if !status.is_client_error() && !status.is_server_error() {
        Ok(resp)
    } else {
        let content = resp.text().await?;
        let entity: Option<DownloadDocError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn download_doc_csv(configuration: &configuration::Configuration, doc_id: &str, table_id: &str, header: Option<&str>) -> Result<String, Error<DownloadDocCsvError>> {
    let uri_str = format!("{config}/docs/{docId}/download/csv",
        config = configuration.base_path,
        docId = crate::apis::urlencode(doc_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("tableId", &table_id.to_string())]);
    if let Some(ref param_value) = header {
        req_builder = req_builder.query(&[("header", &param_value.to_string())]);
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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `String`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `String`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DownloadDocCsvError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn download_doc_xlsx(configuration: &configuration::Configuration, doc_id: &str, header: Option<&str>) -> Result<reqwest::Response, Error<DownloadDocXlsxError>> {
    let uri_str = format!("{config}/docs/{docId}/download/xlsx",
        config = configuration.base_path,
        docId = crate::apis::urlencode(doc_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = header {
        req_builder = req_builder.query(&[("header", &param_value.to_string())]);
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

    if !status.is_client_error() && !status.is_server_error() {
        Ok(resp)
    } else {
        let content = resp.text().await?;
        let entity: Option<DownloadDocXlsxError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// The schema follows [frictionlessdata's table-schema standard](https://specs.frictionlessdata.io/table-schema/).
pub async fn download_table_schema(configuration: &configuration::Configuration, doc_id: &str, table_id: &str, header: Option<&str>) -> Result<models::TableSchemaResult, Error<DownloadTableSchemaError>> {
    let uri_str = format!("{config}/docs/{docId}/download/table-schema",
        config = configuration.base_path,
        docId = crate::apis::urlencode(doc_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("tableId", &table_id.to_string())]);
    if let Some(ref param_value) = header {
        req_builder = req_builder.query(&[("header", &param_value.to_string())]);
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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::TableSchemaResult`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::TableSchemaResult`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DownloadTableSchemaError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Closes and reopens the document, forcing the python engine to restart.
pub async fn force_reload(configuration: &configuration::Configuration, doc_id: &str) -> Result<(), Error<ForceReloadError>> {
    let uri_str = format!("{config}/docs/{docId}/force-reload",
        config = configuration.base_path,
        docId = crate::apis::urlencode(doc_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

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
        let entity: Option<ForceReloadError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn list_doc_access(configuration: &configuration::Configuration, doc_id: &str) -> Result<models::WorkspaceAccessRead, Error<ListDocAccessError>> {
    let uri_str = format!("{config}/docs/{docId}/access",
        config = configuration.base_path,
        docId = crate::apis::urlencode(doc_id)
    );
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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::WorkspaceAccessRead`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::WorkspaceAccessRead`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ListDocAccessError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn modify_doc(configuration: &configuration::Configuration, doc_id: &str, doc_parameters: models::DocParameters) -> Result<(), Error<ModifyDocError>> {
    let uri_str = format!("{config}/docs/{docId}",
        config = configuration.base_path,
        docId = crate::apis::urlencode(doc_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&doc_parameters);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<ModifyDocError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn modify_doc_access(configuration: &configuration::Configuration, doc_id: &str, modify_doc_access_request: models::ModifyDocAccessRequest) -> Result<(), Error<ModifyDocAccessError>> {
    let uri_str = format!("{config}/docs/{docId}/access",
        config = configuration.base_path,
        docId=crate::apis::urlencode(doc_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&modify_doc_access_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<ModifyDocAccessError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn move_doc(configuration: &configuration::Configuration, doc_id: &str, move_doc_request: Option<models::MoveDocRequest>) -> Result<(), Error<MoveDocError>> {
    let uri_str = format!("{config}/docs/{docId}/move",
        config = configuration.base_path,
        docId = crate::apis::urlencode(doc_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&move_doc_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<MoveDocError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

