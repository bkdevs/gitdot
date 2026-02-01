use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct GetDeviceCodeServerRequest {
    pub client_id: String,
}

#[derive(Serialize, PartialEq)]
pub struct DeviceCodeServerResponse {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub expires_in: u64,
    pub interval: u64,
}
