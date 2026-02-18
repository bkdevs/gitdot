use anyhow::Result;

use gitdot_api::endpoint::oauth::{
    authorize_device::AuthorizeDeviceRequest,
    get_device_code::{GetDeviceCodeRequest, GetDeviceCodeResponse},
    poll_token::{PollTokenRequest, PollTokenResponse},
};

use crate::client::GitdotClient;

impl GitdotClient {
    pub async fn get_device_code(
        &self,
        request: GetDeviceCodeRequest,
    ) -> Result<GetDeviceCodeResponse> {
        self.post("oauth/device".to_string(), request).await
    }

    pub async fn poll_token(&self, request: PollTokenRequest) -> Result<PollTokenResponse> {
        self.post("oauth/token".to_string(), request).await
    }

    pub async fn authorize_device(&self, request: AuthorizeDeviceRequest) -> Result<()> {
        self.post("oauth/authorize".to_string(), request).await
    }
}
