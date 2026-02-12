use crate::config::Config;

pub struct ApiClient {
    client: reqwest::Client,
    server_url: String,
}

impl ApiClient {
    pub fn new(config: &Config) -> Self {
        Self {
            client: reqwest::Client::new(),
            server_url: config.gitdot_server_url.clone(),
        }
    }
}
