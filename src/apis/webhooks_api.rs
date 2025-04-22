use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};


/// struct for typed errors of method [`docs_doc_id_webhooks_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DocsDocIdWebhooksGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`docs_doc_id_webhooks_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DocsDocIdWebhooksPostError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`docs_doc_id_webhooks_queue_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DocsDocIdWebhooksQueueDeleteError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`docs_doc_id_webhooks_webhook_id_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DocsDocIdWebhooksWebhookIdDeleteError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`docs_doc_id_webhooks_webhook_id_patch`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DocsDocIdWebhooksWebhookIdPatchError {
    UnknownValue(serde_json::Value),
}


pub async fn docs_doc_id_webhooks_get(configuration: &configuration::Configuration, doc_id: &str) -> Result<models::DocsDocIdWebhooksGet200Response, Error<DocsDocIdWebhooksGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_doc_id = doc_id;

    let uri_str = format!("{}/docs/{docId}/webhooks", configuration.base_path, docId=crate::apis::urlencode(p_doc_id));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DocsDocIdWebhooksGet200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DocsDocIdWebhooksGet200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DocsDocIdWebhooksGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn docs_doc_id_webhooks_post(configuration: &configuration::Configuration, doc_id: &str, docs_doc_id_webhooks_post_request: Option<models::DocsDocIdWebhooksPostRequest>) -> Result<models::DocsDocIdWebhooksPost200Response, Error<DocsDocIdWebhooksPostError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_doc_id = doc_id;
    let p_docs_doc_id_webhooks_post_request = docs_doc_id_webhooks_post_request;

    let uri_str = format!("{}/docs/{docId}/webhooks", configuration.base_path, docId=crate::apis::urlencode(p_doc_id));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_docs_doc_id_webhooks_post_request);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DocsDocIdWebhooksPost200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DocsDocIdWebhooksPost200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DocsDocIdWebhooksPostError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn docs_doc_id_webhooks_queue_delete(configuration: &configuration::Configuration, doc_id: &str) -> Result<(), Error<DocsDocIdWebhooksQueueDeleteError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_doc_id = doc_id;

    let uri_str = format!("{}/docs/{docId}/webhooks/queue", configuration.base_path, docId=crate::apis::urlencode(p_doc_id));
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
        let entity: Option<DocsDocIdWebhooksQueueDeleteError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn docs_doc_id_webhooks_webhook_id_delete(configuration: &configuration::Configuration, doc_id: &str, webhook_id: &str) -> Result<models::DocsDocIdWebhooksWebhookIdDelete200Response, Error<DocsDocIdWebhooksWebhookIdDeleteError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_doc_id = doc_id;
    let p_webhook_id = webhook_id;

    let uri_str = format!("{}/docs/{docId}/webhooks/{webhookId}", configuration.base_path, docId=crate::apis::urlencode(p_doc_id), webhookId=crate::apis::urlencode(p_webhook_id));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DocsDocIdWebhooksWebhookIdDelete200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DocsDocIdWebhooksWebhookIdDelete200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DocsDocIdWebhooksWebhookIdDeleteError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn docs_doc_id_webhooks_webhook_id_patch(configuration: &configuration::Configuration, doc_id: &str, webhook_id: &str, webhook_partial_fields: Option<models::WebhookPartialFields>) -> Result<(), Error<DocsDocIdWebhooksWebhookIdPatchError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_doc_id = doc_id;
    let p_webhook_id = webhook_id;
    let p_webhook_partial_fields = webhook_partial_fields;

    let uri_str = format!("{}/docs/{docId}/webhooks/{webhookId}", configuration.base_path, docId=crate::apis::urlencode(p_doc_id), webhookId=crate::apis::urlencode(p_webhook_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_webhook_partial_fields);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<DocsDocIdWebhooksWebhookIdPatchError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

