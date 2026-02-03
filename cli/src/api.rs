pub mod auth;

const SERVER_URL: &str = "https://gitdot.io";
const CLIENT_ID: &str = "gitdot-cli";

pub struct ApiClient {
    client: reqwest::Client,
    base_url: String,
    client_id: String,
}

impl ApiClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: SERVER_URL.to_string(),
            client_id: CLIENT_ID.to_string(),
        }
    }

    pub fn get_base_url(&self) -> &str {
        &self.base_url
    }
}
