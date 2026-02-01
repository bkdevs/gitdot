use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct PollTokenRequest {
    pub device_code: String,
    pub client_id: String,
}

#[derive(Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub user_name: String,
    pub user_email: String,
}
