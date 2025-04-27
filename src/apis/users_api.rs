use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};


/// struct for typed errors of method [`users_user_id_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UsersUserIdDeleteError {
    Status400(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}


/// This action also deletes the user's personal organisation and all the workspaces and documents it contains. Currently, only the users themselves are allowed to delete their own accounts.  ⚠️ **This action cannot be undone, please be cautious when using this endpoint** ⚠️ 
pub async fn users_user_id_delete(configuration: &configuration::Configuration, user_id: u64, users_user_id_delete_request: Option<models::UsersUserIdDeleteRequest>) -> Result<(), Error<UsersUserIdDeleteError>> {
    let uri_str = format!("{}/users/{userId}", configuration.base_path, userId=user_id);
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&users_user_id_delete_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<UsersUserIdDeleteError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

