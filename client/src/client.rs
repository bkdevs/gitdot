use gitdot_api::endpoint::Endpoint;

const PUBLIC_URL: &str = "https://www.gitdot.io";
const SERVER_URL: &str = "https://api.gitdot.io";

pub struct GitdotClient {
    client: reqwest::Client,
    public_url: String,
    server_url: String,
    client_id: String,
}

impl GitdotClient {
    pub fn new(client_id: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            public_url: PUBLIC_URL.to_string(),
            server_url: SERVER_URL.to_string(),
            client_id,
        }
    }

    pub fn get_server_url(&self) -> &str {
        &self.server_url
    }

    // async fn post<T, R>(&self, request: T) -> Result<R, Error>
    // where
    //     T: Endpoint::Request,
    //     R: Endpoint::Response,
    //     Error: From<reqwest::Error>,
    //     Error: From<serde_json::Error>,
    // {
    //     let url = format!("{}/{}", self.server_url, request.path());
    //     let response = self.client.post(&url).json(&request).send().await?;
    //     response.json().await
    // }
}
