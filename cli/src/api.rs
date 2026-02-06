pub mod auth;

const PUBLIC_URL: &str = "https://www.gitdot.io";
const SERVER_URL: &str = "https://api.gitdot.io";
const CLIENT_ID: &str = "gitdot-cli";

pub struct ApiClient {
    client: reqwest::Client,
    public_url: String,
    server_url: String,
    client_id: String,
}

impl ApiClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            public_url: PUBLIC_URL.to_string(),
            server_url: SERVER_URL.to_string(),
            client_id: CLIENT_ID.to_string(),
        }
    }

    pub fn get_public_url(&self) -> &str {
        &self.public_url
    }
}
