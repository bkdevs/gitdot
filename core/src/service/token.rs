use async_trait::async_trait;
use chrono::{Duration, Utc};

use crate::{
    dto::{
        AuthorizeDeviceRequest, DeviceCodeRequest, DeviceCodeResponse, PollTokenRequest,
        TokenResponse, ValidateTokenRequest, ValidateTokenResponse,
    },
    error::TokenError,
    model::{DeviceAuthorizationStatus, TokenType},
    repository::{TokenRepository, TokenRepositoryImpl, UserRepository, UserRepositoryImpl},
    util::token::{
        DEVICE_CODE_EXPIRY_MINUTES, POLLING_INTERVAL_SECONDS, generate_access_token,
        generate_device_code, generate_user_code, hash_token, validate_token_format,
    },
};

#[async_trait]
pub trait TokenService: Send + Sync + 'static {
    async fn request_device_code(
        &self,
        request: DeviceCodeRequest,
    ) -> Result<DeviceCodeResponse, TokenError>;

    async fn poll_token(&self, request: PollTokenRequest) -> Result<TokenResponse, TokenError>;

    async fn authorize_device(&self, request: AuthorizeDeviceRequest) -> Result<(), TokenError>;

    async fn validate_token(
        &self,
        request: ValidateTokenRequest,
    ) -> Result<ValidateTokenResponse, TokenError>;
}

#[derive(Debug, Clone)]
pub struct TokenServiceImpl<R, U>
where
    R: TokenRepository,
    U: UserRepository,
{
    token_repo: R,
    user_repo: U,
}

impl TokenServiceImpl<TokenRepositoryImpl, UserRepositoryImpl> {
    pub fn new(token_repo: TokenRepositoryImpl, user_repo: UserRepositoryImpl) -> Self {
        Self {
            token_repo,
            user_repo,
        }
    }
}

#[async_trait]
impl<R, U> TokenService for TokenServiceImpl<R, U>
where
    R: TokenRepository,
    U: UserRepository,
{
    async fn request_device_code(
        &self,
        request: DeviceCodeRequest,
    ) -> Result<DeviceCodeResponse, TokenError> {
        let device_code = generate_device_code();
        let user_code = generate_user_code();
        let expires_at = Utc::now() + Duration::minutes(DEVICE_CODE_EXPIRY_MINUTES);

        self.token_repo
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

    async fn poll_token(&self, request: PollTokenRequest) -> Result<TokenResponse, TokenError> {
        let device_auth = self
            .token_repo
            .get_device_authorization_by_device_code(&request.device_code)
            .await?
            .ok_or(TokenError::InvalidDeviceCode)?;

        if device_auth.expires_at < Utc::now()
            && device_auth.status == DeviceAuthorizationStatus::Pending
        {
            self.token_repo
                .expire_device_authorization(device_auth.id)
                .await?;
            return Err(TokenError::ExpiredToken);
        }

        match device_auth.status {
            DeviceAuthorizationStatus::Pending => {
                return Err(TokenError::AuthorizationPending);
            }
            DeviceAuthorizationStatus::Expired => {
                return Err(TokenError::ExpiredToken);
            }
            DeviceAuthorizationStatus::Authorized => {
                let user_id = device_auth
                    .user_id
                    .ok_or(TokenError::InvalidRequest("Missing user_id".to_string()))?;

                let user = self
                    .user_repo
                    .get_by_id(user_id)
                    .await
                    .map_err(|e| TokenError::InvalidRequest(e.to_string()))?
                    .ok_or(TokenError::InvalidRequest("User not found".to_string()))?;

                let access_token = generate_access_token(&TokenType::Personal);
                let token_hash = hash_token(&access_token);

                self.token_repo
                    .create_access_token(user_id, &device_auth.client_id, &token_hash)
                    .await?;

                self.token_repo
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

    async fn authorize_device(&self, request: AuthorizeDeviceRequest) -> Result<(), TokenError> {
        self.token_repo
            .authorize_device(&request.user_code, request.user_id)
            .await?
            .ok_or(TokenError::InvalidUserCode(
                "User code not found".to_string(),
            ))?;

        Ok(())
    }

    async fn validate_token(
        &self,
        request: ValidateTokenRequest,
    ) -> Result<ValidateTokenResponse, TokenError> {
        if !validate_token_format(&request.token) {
            return Err(TokenError::AccessDenied);
        }

        let token_hash = hash_token(&request.token);

        let access_token = self
            .token_repo
            .get_access_token_by_hash(&token_hash)
            .await?
            .ok_or(TokenError::AccessDenied)?;

        self.token_repo.touch_access_token(access_token.id).await?;

        Ok(ValidateTokenResponse {
            user_id: access_token.user_id,
        })
    }
}
