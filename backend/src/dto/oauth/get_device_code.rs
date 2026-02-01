use serde::{Deserialize, Serialize};

use gitdot_core::dto::DeviceCodeResponse;

#[derive(Deserialize)]
pub struct GetDeviceCodeQuery {
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

impl From<DeviceCodeResponse> for DeviceCodeServerResponse {
    fn from(response: DeviceCodeResponse) -> Self {
        Self {
            device_code: response.device_code,
            user_code: response.user_code,
            verification_uri: response.verification_uri,
            expires_in: response.expires_in,
            interval: response.interval,
        }
    }
}
