pub mod auth;

const API_ENDPOINT: &str = "http://localhost:8080";
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
            base_url: API_ENDPOINT.to_string(),
            client_id: CLIENT_ID.to_string(),
        }
    }

    pub fn get_base_url(&self) -> &str {
        &self.base_url
    }
}
