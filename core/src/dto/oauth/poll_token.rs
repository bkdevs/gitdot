#[derive(Debug, Clone)]
pub struct PollTokenRequest {
    pub device_code: String,
    pub client_id: String,
}

impl PollTokenRequest {
    pub fn new(device_code: String, client_id: String) -> Self {
        Self {
            device_code,
            client_id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TokenResponse {
    pub access_token: String,
}
