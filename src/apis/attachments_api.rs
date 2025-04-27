use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};


/// struct for typed errors of method [`download_attachment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DownloadAttachmentError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`download_attachments`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DownloadAttachmentsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_attachment_metadata`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAttachmentMetadataError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_attachment_transfer_status`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAttachmentTransferStatusError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_document_attachment_store`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDocumentAttachmentStoreError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_attachment_stores`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAttachmentStoresError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_attachments`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAttachmentsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_document_attachment_store`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetDocumentAttachmentStoreError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`start_attachment_transfer`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StartAttachmentTransferError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`upload_attachments`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UploadAttachmentsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`upload_missing_attachments`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UploadMissingAttachmentsError {
    UnknownValue(serde_json::Value),
}

pub async fn download_attachment(configuration: &configuration::Configuration, doc_id: &str, attachment_id: u64) -> Result<Vec<u8>, Error<DownloadAttachmentError>> {
    // Construct the URI string for the request
    let uri_str = format!("{base_path}/docs/{docId}/attachments/{attachmentId}/download",
        base_path = configuration.base_path, 
        docId = crate::apis::urlencode(doc_id),
        attachmentId = attachment_id
    );

    // Build the request
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    }

    // Execute the request
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;
    let status = resp.status();

    // Check for error responses
    if status.is_success() {
        // Successfully received response, read the binary data.
        let data = resp.bytes().await?.to_vec();
        Ok(data)
    } else {
        // Handle error responses
        let content = resp.text().await?;
        let entity: Option<DownloadAttachmentError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn download_attachments(configuration: &configuration::Configuration, doc_id: &str, format: Option<&str>) -> Result<(), Error<DownloadAttachmentsError>> {
    let uri_str = format!("{config}/docs/{docId}/attachments/archive",
        config = configuration.base_path,
        docId = crate::apis::urlencode(doc_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = format {
        req_builder = req_builder.query(&[("format", &param_value.to_string())]);
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
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<DownloadAttachmentsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_attachment_metadata(configuration: &configuration::Configuration, doc_id: &str, attachment_id: f64) -> Result<models::AttachmentMetadata, Error<GetAttachmentMetadataError>> {
    let uri_str = format!("{config}/docs/{docId}/attachments/{attachmentId}",
        config = configuration.base_path,
        docId=crate::apis::urlencode(doc_id),
        attachmentId=attachment_id
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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AttachmentMetadata`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AttachmentMetadata`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetAttachmentMetadataError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_attachment_transfer_status(configuration: &configuration::Configuration, doc_id: &str) -> Result<models::AttachmentsTransferStatus, Error<GetAttachmentTransferStatusError>> {
    let uri_str = format!("{base_path}/api/docs/{docId}/attachments/transferStatus",
        base_path = configuration.base_path,
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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AttachmentsTransferStatus`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AttachmentsTransferStatus`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetAttachmentTransferStatusError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_document_attachment_store(configuration: &configuration::Configuration, doc_id: &str) -> Result<models::DocumentStoreSetting, Error<GetDocumentAttachmentStoreError>> {
    let uri_str = format!("{base_path}/docs/{docId}/attachments/store",
        base_path = configuration.base_path,
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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DocumentStoreSetting`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DocumentStoreSetting`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetDocumentAttachmentStoreError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn list_attachment_stores(configuration: &configuration::Configuration, doc_id: &str) -> Result<models::DocumentStoreSetting, Error<ListAttachmentStoresError>> {
    let uri_str = format!("{base_path}/docs/{docId}/attachments/stores",
        base_path = configuration.base_path,
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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DocumentStoreSetting`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DocumentStoreSetting`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ListAttachmentStoresError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn list_attachments(configuration: &configuration::Configuration, doc_id: &str, filter: Option<&str>, sort: Option<&str>, limit: Option<f64>, x_sort: Option<&str>, x_limit: Option<f64>) -> Result<models::AttachmentMetadataList, Error<ListAttachmentsError>> {
    let uri_str = format!("{base_path}/docs/{docId}/attachments",
        base_path = configuration.base_path,
        docId = crate::apis::urlencode(doc_id)
    );
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
    if let Some(param_value) = x_sort {
        req_builder = req_builder.header("X-Sort", param_value.to_string());
    }
    if let Some(param_value) = x_limit {
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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AttachmentMetadataList`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AttachmentMetadataList`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ListAttachmentsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn set_document_attachment_store(configuration: &configuration::Configuration, doc_id: &str, document_store_setting: Option<models::DocumentStoreSetting>) -> Result<models::SetDocumentAttachmentStore200Response, Error<SetDocumentAttachmentStoreError>> {
    let uri_str = format!("{base_path}/docs/{docId}/attachments/store",
        base_path = configuration.base_path,
        docId = crate::apis::urlencode(doc_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&document_store_setting);

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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::SetDocumentAttachmentStore200Response`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::SetDocumentAttachmentStore200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SetDocumentAttachmentStoreError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn start_attachment_transfer(configuration: &configuration::Configuration, doc_id: &str) -> Result<models::AttachmentsTransferStatus, Error<StartAttachmentTransferError>> {
    let uri_str = format!("{base_path}/api/docs/{docId}/attachments/transferAll",
        base_path = configuration.base_path,
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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AttachmentsTransferStatus`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AttachmentsTransferStatus`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<StartAttachmentTransferError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn upload_attachments(configuration: &configuration::Configuration, doc_id: &str, upload: Vec<std::path::PathBuf>) -> Result<Vec<u64>, Error<UploadAttachmentsError>> {
    let uri_str = format!("{base_path}/docs/{docId}/attachments",
        base_path = configuration.base_path,
        docId = crate::apis::urlencode(doc_id)
    );

    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    }

    let mut multipart_form = reqwest::multipart::Form::new();

    for path in upload {
        let file_name = path.file_name()
            .and_then(|os_str| os_str.to_str())
            .expect("Failed to get file name")
            .to_owned();

        // Add the file to the multipart form
        let file_part = reqwest::multipart::Part::file(path).await.expect("Failed to load file into multipart").file_name(file_name);
        multipart_form = multipart_form.part("upload", file_part);
    }

    req_builder = req_builder.multipart(multipart_form);
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;
    let status = resp.status();

    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if status.is_success() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `Vec<i32>`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `Vec<i32>`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UploadAttachmentsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Restores attachments which are missing from external storage.
pub async fn upload_missing_attachments(configuration: &configuration::Configuration, doc_id: &str, file: Option<std::path::PathBuf>) -> Result<models::UploadMissingAttachments200Response, Error<UploadMissingAttachmentsError>> {
    let uri_str = format!("{config}/docs/{docId}/attachments/archive",
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
    let mut multipart_form = reqwest::multipart::Form::new();

    if let Some(path) = file {
        let file_name = path.file_name()
            .and_then(|os_str| os_str.to_str())
            .expect("Failed to get file name")
            .to_owned();

        // Add the file to the multipart form
        let file_part = reqwest::multipart::Part::file(path).await.expect("Failed to load file into multipart").file_name(file_name);
        multipart_form = multipart_form.part("upload", file_part);
    }
    
    req_builder = req_builder.multipart(multipart_form);

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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::UploadMissingAttachments200Response`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::UploadMissingAttachments200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UploadMissingAttachmentsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

