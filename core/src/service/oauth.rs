use async_trait::async_trait;
use chrono::{Duration, Utc};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::dto::{
    AuthorizeDeviceRequest, DeviceCodeRequest, DeviceCodeResponse, PollTokenRequest, TokenResponse,
};
use crate::error::OAuthError;
use crate::repository::{OAuthRepository, OAuthRepositoryImpl};

const DEVICE_CODE_EXPIRY_MINUTES: i64 = 15;
const POLLING_INTERVAL_SECONDS: u64 = 5;

#[async_trait]
pub trait OAuthService: Send + Sync + 'static {
    async fn request_device_code(
        &self,
        request: DeviceCodeRequest,
        verification_uri: &str,
    ) -> Result<DeviceCodeResponse, OAuthError>;

    async fn poll_token(&self, request: PollTokenRequest) -> Result<TokenResponse, OAuthError>;

    async fn authorize_device(&self, request: AuthorizeDeviceRequest) -> Result<(), OAuthError>;

    async fn validate_token(&self, token: &str) -> Result<Uuid, OAuthError>;
}

#[derive(Debug, Clone)]
pub struct OAuthServiceImpl<R>
where
    R: OAuthRepository,
{
    oauth_repo: R,
}

impl OAuthServiceImpl<OAuthRepositoryImpl> {
    pub fn new(oauth_repo: OAuthRepositoryImpl) -> Self {
        Self { oauth_repo }
    }
}

#[async_trait]
impl<R> OAuthService for OAuthServiceImpl<R>
where
    R: OAuthRepository,
{
    async fn request_device_code(
        &self,
        _request: DeviceCodeRequest,
        verification_uri: &str,
    ) -> Result<DeviceCodeResponse, OAuthError> {
        let device_code = generate_device_code();
        let user_code = generate_user_code();
        let expires_at = Utc::now() + Duration::minutes(DEVICE_CODE_EXPIRY_MINUTES);

        self.oauth_repo
            .create_device_authorization(&device_code, &user_code, expires_at)
            .await?;

        Ok(DeviceCodeResponse {
            device_code,
            user_code,
            verification_uri: verification_uri.to_string(),
            expires_in: (DEVICE_CODE_EXPIRY_MINUTES * 60) as u64,
            interval: POLLING_INTERVAL_SECONDS,
        })
    }

    async fn poll_token(&self, request: PollTokenRequest) -> Result<TokenResponse, OAuthError> {
        let device_auth = self
            .oauth_repo
            .get_device_authorization_by_device_code(&request.device_code)
            .await?
            .ok_or(OAuthError::InvalidDeviceCode)?;

        // Check if expired
        if device_auth.expires_at < Utc::now() {
            return Err(OAuthError::ExpiredToken);
        }

        match device_auth.status.as_str() {
            "pending" => Err(OAuthError::AuthorizationPending),
            "denied" => Err(OAuthError::AccessDenied),
            "authorized" => {
                let user_id = device_auth
                    .user_id
                    .ok_or(OAuthError::InvalidRequest("Missing user_id".to_string()))?;

                // Generate access token
                let access_token = generate_access_token();
                let token_hash = hash_token(&access_token);

                self.oauth_repo
                    .create_access_token(user_id, &token_hash, Some("CLI"))
                    .await?;

                Ok(TokenResponse {
                    access_token,
                    token_type: "bearer".to_string(),
                })
            }
            _ => Err(OAuthError::InvalidRequest(format!(
                "Unknown status: {}",
                device_auth.status
            ))),
        }
    }

    async fn authorize_device(&self, request: AuthorizeDeviceRequest) -> Result<(), OAuthError> {
        let device_auth = self
            .oauth_repo
            .authorize_device(&request.user_code, request.user_id)
            .await?;

        if device_auth.is_none() {
            // Check if it exists but is expired or already used
            let existing = self
                .oauth_repo
                .get_device_authorization_by_user_code(&request.user_code)
                .await?;

            return match existing {
                None => Err(OAuthError::InvalidUserCode),
                Some(auth) if auth.expires_at < Utc::now() => Err(OAuthError::ExpiredToken),
                Some(_) => Err(OAuthError::InvalidRequest(
                    "Device already authorized or denied".to_string(),
                )),
            };
        }

        Ok(())
    }

    async fn validate_token(&self, token: &str) -> Result<Uuid, OAuthError> {
        let token_hash = hash_token(token);

        let access_token = self
            .oauth_repo
            .get_access_token_by_hash(&token_hash)
            .await?
            .ok_or(OAuthError::InvalidRequest("Invalid token".to_string()))?;

        // Update last used timestamp
        self.oauth_repo.touch_access_token(access_token.id).await?;

        Ok(access_token.user_id)
    }
}

fn generate_device_code() -> String {
    use rand::Rng as _;
    let mut rng = rand::thread_rng();
    (0..32)
        .map(|_| {
            let idx = rng.gen_range(0..36);
            if idx < 10 {
                (b'0' + idx) as char
            } else {
                (b'a' + idx - 10) as char
            }
        })
        .collect()
}

fn generate_user_code() -> String {
    use rand::Rng as _;
    let mut rng = rand::thread_rng();
    let chars: Vec<char> = "ABCDEFGHJKLMNPQRSTUVWXYZ23456789".chars().collect();
    let part1: String = (0..4)
        .map(|_| chars[rng.gen_range(0..chars.len())])
        .collect();
    let part2: String = (0..4)
        .map(|_| chars[rng.gen_range(0..chars.len())])
        .collect();
    format!("{}-{}", part1, part2)
}

fn generate_access_token() -> String {
    use rand::Rng as _;
    let mut rng = rand::thread_rng();
    let bytes: [u8; 32] = rng.r#gen();
    hex::encode(bytes)
}

pub fn hash_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    hex::encode(hasher.finalize())
}
