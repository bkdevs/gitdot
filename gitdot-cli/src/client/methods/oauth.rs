use anyhow::Result;

use gitdot_api::endpoint::oauth::{
    create_device_code::{CreateDeviceCodeRequest, CreateDeviceCodeResponse},
    poll_token::{PollTokenRequest, PollTokenResponse},
};

use crate::client::GitdotClient;

impl GitdotClient {
    pub async fn create_device_code(&self) -> Result<CreateDeviceCodeResponse> {
        let request = CreateDeviceCodeRequest {
            client_id: self.get_client_id().to_string(),
        };
        self.post("oauth/device".to_string(), request).await
    }

    pub async fn poll_token(&self, device_code: &str) -> Result<PollTokenResponse> {
        let request = PollTokenRequest {
            client_id: self.get_client_id().to_string(),
            device_code: device_code.to_string(),
        };
        self.post("oauth/token".to_string(), request).await
    }
}
