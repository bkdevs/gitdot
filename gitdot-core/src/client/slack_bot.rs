use async_trait::async_trait;
use chrono::Utc;
use hmac::{Hmac, Mac};
use reqwest::{
    Client,
    header::{HeaderMap, HeaderValue},
};
use serde::Serialize;
use sha2::Sha256;
use uuid::Uuid;

use crate::error::SlackBotError;

const TIMESTAMP_HEADER: &str = "x-gitdot-timestamp";
const SIGNATURE_HEADER: &str = "x-gitdot-signature";

const FINALIZE_LOGIN_PATH: &str = "/gitdot/auth/finalize";

#[async_trait]
pub trait SlackBotClient: Send + Sync + Clone + 'static {
    async fn notify_link_completed(
        &self,
        gitdot_user_id: Uuid,
        gitdot_user_name: &str,
        channel_id: &str,
    ) -> Result<(), SlackBotError>;
}

#[derive(Debug, Clone)]
pub struct SlackBotClientImpl {
    http: Client,
    server_url: String,
    slack_secret: String,
}

impl SlackBotClientImpl {
    pub fn new(server_url: String, slack_secret: String) -> Self {
        Self {
            http: Client::new(),
            server_url,
            slack_secret,
        }
    }

    fn sign(&self, timestamp: i64, body: &[u8]) -> String {
        let mut mac = Hmac::<Sha256>::new_from_slice(self.slack_secret.as_bytes())
            .expect("HMAC accepts any key length");
        mac.update(b"v0:");
        mac.update(timestamp.to_string().as_bytes());
        mac.update(b":");
        mac.update(body);
        format!("v0={}", hex::encode(mac.finalize().into_bytes()))
    }

    async fn post<T: Serialize>(&self, path: &str, body: &T) -> Result<(), SlackBotError> {
        let body_bytes = serde_json::to_vec(body)?;
        let timestamp = Utc::now().timestamp();
        let signature = self.sign(timestamp, &body_bytes);

        let mut headers = HeaderMap::new();
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        headers.insert(
            TIMESTAMP_HEADER,
            HeaderValue::from_str(&timestamp.to_string())
                .expect("ascii numeric timestamp is a valid header value"),
        );
        headers.insert(
            SIGNATURE_HEADER,
            HeaderValue::from_str(&signature)
                .expect("hex-encoded signature is a valid header value"),
        );

        let url = format!("{}{}", self.server_url, path);
        let response = self
            .http
            .post(&url)
            .headers(headers)
            .body(body_bytes)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            tracing::warn!(
                status = status.as_u16(),
                url = %url,
                body = %body,
                "slack_bot: non-success response",
            );
            return Err(SlackBotError::NonSuccessStatus {
                status: status.as_u16(),
                body,
            });
        }

        Ok(())
    }
}

#[derive(Serialize)]
struct FinalizeLoginRequest<'a> {
    gitdot_user_id: Uuid,
    gitdot_user_name: &'a str,
    channel_id: &'a str,
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl SlackBotClient for SlackBotClientImpl {
    async fn notify_link_completed(
        &self,
        gitdot_user_id: Uuid,
        gitdot_user_name: &str,
        channel_id: &str,
    ) -> Result<(), SlackBotError> {
        self.post(
            FINALIZE_LOGIN_PATH,
            &FinalizeLoginRequest {
                gitdot_user_id,
                gitdot_user_name,
                channel_id,
            },
        )
        .await
    }
}
