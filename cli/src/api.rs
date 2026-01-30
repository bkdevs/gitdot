use anyhow::{Context, Result, bail};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Deserialize)]
pub struct DeviceCodeResponse {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub expires_in: u64,
    pub interval: u64,
}

#[derive(Debug, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
}

#[derive(Debug, Deserialize)]
pub struct OAuthError {
    pub error: String,
    pub error_description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
}

pub async fn request_device_code(api_url: &str) -> Result<DeviceCodeResponse> {
    let client = Client::new();
    let url = format!("{}/oauth/device", api_url);

    let response = client
        .post(&url)
        .send()
        .await
        .context("Failed to request device code")?;

    if !response.status().is_success() {
        let error: OAuthError = response.json().await.unwrap_or(OAuthError {
            error: "unknown".to_string(),
            error_description: Some("Failed to request device code".to_string()),
        });
        bail!(
            "Failed to request device code: {}",
            error.error_description.unwrap_or(error.error)
        );
    }

    response
        .json()
        .await
        .context("Failed to parse device code response")
}

#[derive(Serialize)]
struct PollTokenRequest {
    device_code: String,
}

pub enum PollResult {
    Token(TokenResponse),
    Pending,
    SlowDown,
    Expired,
    AccessDenied,
}

pub async fn poll_for_token(api_url: &str, device_code: &str) -> Result<PollResult> {
    let client = Client::new();
    let url = format!("{}/oauth/token", api_url);

    let response = client
        .post(&url)
        .json(&PollTokenRequest {
            device_code: device_code.to_string(),
        })
        .send()
        .await
        .context("Failed to poll for token")?;

    if response.status().is_success() {
        let token: TokenResponse = response
            .json()
            .await
            .context("Failed to parse token response")?;
        return Ok(PollResult::Token(token));
    }

    let error: OAuthError = response.json().await.unwrap_or(OAuthError {
        error: "unknown".to_string(),
        error_description: None,
    });

    match error.error.as_str() {
        "authorization_pending" => Ok(PollResult::Pending),
        "slow_down" => Ok(PollResult::SlowDown),
        "expired_token" => Ok(PollResult::Expired),
        "access_denied" => Ok(PollResult::AccessDenied),
        _ => bail!("Token polling failed: {}", error.error),
    }
}

pub async fn poll_until_authorized(
    api_url: &str,
    device_code: &str,
    interval: u64,
) -> Result<TokenResponse> {
    let mut current_interval = Duration::from_secs(interval);

    loop {
        tokio::time::sleep(current_interval).await;

        match poll_for_token(api_url, device_code).await? {
            PollResult::Token(token) => return Ok(token),
            PollResult::Pending => {
                // Continue polling
            }
            PollResult::SlowDown => {
                // Increase interval by 5 seconds
                current_interval += Duration::from_secs(5);
            }
            PollResult::Expired => {
                bail!("Device code expired. Please try again.");
            }
            PollResult::AccessDenied => {
                bail!("Authorization was denied.");
            }
        }
    }
}

pub async fn get_current_user(api_url: &str, token: &str) -> Result<User> {
    let client = Client::new();
    let url = format!("{}/user", api_url);

    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .context("Failed to get current user")?;

    if !response.status().is_success() {
        bail!("Failed to get user info: {}", response.status());
    }

    response
        .json()
        .await
        .context("Failed to parse user response")
}
