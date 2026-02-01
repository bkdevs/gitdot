#[derive(Debug, Clone)]
pub struct DeviceCodeRequest {
    pub client_id: String,
}

impl DeviceCodeRequest {
    pub fn new(client_id: String) -> Self {
        Self { client_id }
    }
}

#[derive(Debug, Clone)]
pub struct DeviceCodeResponse {
    pub device_code: String,
    pub user_code: String,
    pub expires_in: u64,
    pub interval: u64,
}
