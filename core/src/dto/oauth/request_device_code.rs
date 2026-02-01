#[derive(Debug, Clone)]
pub struct DeviceCodeRequest {
    pub client_id: String,
    pub verification_uri: String,
}

impl DeviceCodeRequest {
    pub fn new(client_id: String, verification_uri: String) -> Self {
        Self {
            client_id,
            verification_uri,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DeviceCodeResponse {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub expires_in: u64,
    pub interval: u64,
}
