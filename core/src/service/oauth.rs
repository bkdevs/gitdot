use async_trait::async_trait;
use chrono::{Duration, Utc};
use uuid::Uuid;

use crate::dto::{
    AuthorizeDeviceRequest, DeviceCodeRequest, DeviceCodeResponse, PollTokenRequest, TokenResponse,
};
use crate::error::OAuthError;
use crate::model::DeviceAuthorizationStatus;
use crate::repository::{OAuthRepository, OAuthRepositoryImpl, UserRepository, UserRepositoryImpl};
use crate::util::oauth::{
    DEVICE_CODE_EXPIRY_MINUTES, POLLING_INTERVAL_SECONDS, generate_access_token,
    generate_device_code, generate_user_code, hash_token,
};

#[async_trait]
pub trait OAuthService: Send + Sync + 'static {
    async fn request_device_code(
        &self,
        request: DeviceCodeRequest,
    ) -> Result<DeviceCodeResponse, OAuthError>;

    async fn poll_token(&self, request: PollTokenRequest) -> Result<TokenResponse, OAuthError>;

    async fn authorize_device(&self, request: AuthorizeDeviceRequest) -> Result<(), OAuthError>;

    async fn validate_token(&self, token: &str) -> Result<Uuid, OAuthError>;
}

#[derive(Debug, Clone)]
pub struct OAuthServiceImpl<R, U>
where
    R: OAuthRepository,
    U: UserRepository,
{
    oauth_repo: R,
    user_repo: U,
}

impl OAuthServiceImpl<OAuthRepositoryImpl, UserRepositoryImpl> {
    pub fn new(oauth_repo: OAuthRepositoryImpl, user_repo: UserRepositoryImpl) -> Self {
        Self {
            oauth_repo,
            user_repo,
        }
    }
}

#[async_trait]
impl<R, U> OAuthService for OAuthServiceImpl<R, U>
where
    R: OAuthRepository,
    U: UserRepository,
{
    async fn request_device_code(
        &self,
        request: DeviceCodeRequest,
    ) -> Result<DeviceCodeResponse, OAuthError> {
        let device_code = generate_device_code();
        let user_code = generate_user_code();
        let expires_at = Utc::now() + Duration::minutes(DEVICE_CODE_EXPIRY_MINUTES);

        self.oauth_repo
            .create_device_authorization(&device_code, &user_code, &request.client_id, expires_at)
            .await?;

        Ok(DeviceCodeResponse {
            device_code,
            user_code,
            verification_uri: request.verification_uri,
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

        if device_auth.expires_at < Utc::now()
            && device_auth.status == DeviceAuthorizationStatus::Pending
        {
            self.oauth_repo
                .expire_device_authorization(device_auth.id)
                .await?;
            return Err(OAuthError::ExpiredToken);
        }

        match device_auth.status {
            DeviceAuthorizationStatus::Pending => {
                return Err(OAuthError::AuthorizationPending);
            }
            DeviceAuthorizationStatus::Expired => {
                return Err(OAuthError::ExpiredToken);
            }
            DeviceAuthorizationStatus::Authorized => {
                let user_id = device_auth
                    .user_id
                    .ok_or(OAuthError::InvalidRequest("Missing user_id".to_string()))?;

                let user = self
                    .user_repo
                    .get_by_id(user_id)
                    .await
                    .map_err(|e| OAuthError::InvalidRequest(e.to_string()))?
                    .ok_or(OAuthError::InvalidRequest("User not found".to_string()))?;

                let access_token = generate_access_token();
                let token_hash = hash_token(&access_token);

                self.oauth_repo
                    .create_access_token(user_id, &device_auth.client_id, &token_hash)
                    .await?;

                self.oauth_repo
                    .expire_device_authorization(device_auth.id)
                    .await?;

                Ok(TokenResponse {
                    access_token,
                    user_name: user.name,
                    user_email: user.email,
                })
            }
        }
    }

    async fn authorize_device(&self, request: AuthorizeDeviceRequest) -> Result<(), OAuthError> {
        self.oauth_repo
            .authorize_device(&request.user_code, request.user_id)
            .await?
            .ok_or(OAuthError::InvalidUserCode)?;

        Ok(())
    }

    async fn validate_token(&self, token: &str) -> Result<Uuid, OAuthError> {
        let token_hash = hash_token(token);

        let access_token = self
            .oauth_repo
            .get_access_token_by_hash(&token_hash)
            .await?
            .ok_or(OAuthError::AccessDenied)?;

        self.oauth_repo.touch_access_token(access_token.id).await?;

        Ok(access_token.user_id)
    }
}
