const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Clone)]
pub struct Configuration {
    pub base_path: String,
    pub user_agent: Option<String>,
    pub client: reqwest::Client,
    pub bearer_access_token: Option<String>,
}

impl Configuration {
    pub fn new(base_path: String, bearer_access_token: Option<String>) -> Configuration {
        Configuration {
			base_path,
            user_agent: Some(format!("{NAME}/{VERSION}")),
			client: Default::default(),
            bearer_access_token
        }
    }
}