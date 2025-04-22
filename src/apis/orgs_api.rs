use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};


/// struct for typed errors of method [`delete_org`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteOrgError {
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`describe_org`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DescribeOrgError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_org_access`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListOrgAccessError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_orgs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListOrgsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`modify_org`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ModifyOrgError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`modify_org_access`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ModifyOrgAccessError {
    UnknownValue(serde_json::Value),
}


pub async fn delete_org(configuration: &configuration::Configuration, org_id: &str) -> Result<(), Error<DeleteOrgError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_org_id = org_id;

    let uri_str = format!("{}/orgs/{orgId}", configuration.base_path, orgId=p_org_id.to_string());
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
        let entity: Option<DeleteOrgError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn describe_org(configuration: &configuration::Configuration, org_id: &str) -> Result<models::Org, Error<DescribeOrgError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_org_id = org_id;

    let uri_str = format!("{}/orgs/{orgId}", configuration.base_path, orgId=p_org_id.to_string());
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Org`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Org`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DescribeOrgError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn list_org_access(configuration: &configuration::Configuration, org_id: &str) -> Result<models::OrgAccessRead, Error<ListOrgAccessError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_org_id = org_id;

    let uri_str = format!("{}/orgs/{orgId}/access", configuration.base_path, orgId=p_org_id.to_string());
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::OrgAccessRead`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::OrgAccessRead`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ListOrgAccessError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// This enumerates all the team sites or personal areas available.
pub async fn list_orgs(configuration: &configuration::Configuration, ) -> Result<Vec<models::Org>, Error<ListOrgsError>> {

    let uri_str = format!("{}/orgs", configuration.base_path);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `Vec&lt;models::Org&gt;`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `Vec&lt;models::Org&gt;`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ListOrgsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn modify_org(configuration: &configuration::Configuration, org_id: &str, org_parameters: models::OrgParameters) -> Result<(), Error<ModifyOrgError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_org_id = org_id;
    let p_org_parameters = org_parameters;

    let uri_str = format!("{}/orgs/{orgId}", configuration.base_path, orgId=p_org_id.to_string());
    let mut req_builder = configuration.client.request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_org_parameters);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<ModifyOrgError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn modify_org_access(configuration: &configuration::Configuration, org_id: &str, modify_org_access_request: models::ModifyOrgAccessRequest) -> Result<(), Error<ModifyOrgAccessError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_org_id = org_id;
    let p_modify_org_access_request = modify_org_access_request;

    let uri_str = format!("{}/orgs/{orgId}/access", configuration.base_path, orgId=p_org_id.to_string());
    let mut req_builder = configuration.client.request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_modify_org_access_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<ModifyOrgAccessError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

