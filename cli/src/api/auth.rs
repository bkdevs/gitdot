mod get_device_code;
mod poll_token;

pub use get_device_code::DeviceCodeResponse;
pub use poll_token::{PollTokenRequest, TokenResponse};

use crate::api::ApiClient;

impl ApiClient {
    pub async fn get_device_code(&self) -> anyhow::Result<DeviceCodeResponse> {
        let url = format!(
            "{}/oauth/device?client_id={}",
            self.server_url, self.client_id
        );
        let response = self
            .client
            .post(&url)
            .send()
            .await?
            .error_for_status()?
            .json::<DeviceCodeResponse>()
            .await?;

        Ok(response)
    }

    pub async fn poll_token(&self, device_code: &str) -> anyhow::Result<TokenResponse> {
        let url = format!("{}/oauth/token", self.server_url);
        let request = PollTokenRequest {
            device_code: device_code.to_string(),
            client_id: self.client_id.clone(),
        };
        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await?
            .error_for_status()?
            .json::<TokenResponse>()
            .await?;

        Ok(response)
    }
}
