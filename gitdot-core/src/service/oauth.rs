use async_trait::async_trait;
use chrono::{Duration, Utc};

use crate::{
    client::{TokenClient, TokenClientImpl},
    dto::{
        AuthorizeDeviceRequest, DeviceCodeRequest, DeviceCodeResponse, PollTokenRequest,
        TokenResponse,
    },
    error::TokenError,
    model::{DeviceAuthorizationStatus, TokenType},
    repository::{
        CodeRepository, CodeRepositoryImpl, TokenRepository, TokenRepositoryImpl, UserRepository,
        UserRepositoryImpl,
    },
    util::crypto::hash_string,
};

#[async_trait]
pub trait OAuthService: Send + Sync + 'static {
    async fn request_device_code(
        &self,
        request: DeviceCodeRequest,
    ) -> Result<DeviceCodeResponse, TokenError>;

    async fn poll_token(&self, request: PollTokenRequest) -> Result<TokenResponse, TokenError>;

    async fn authorize_device(&self, request: AuthorizeDeviceRequest) -> Result<(), TokenError>;
}

#[derive(Debug, Clone)]
pub struct OAuthServiceImpl<D, T, U, TC>
where
    D: CodeRepository,
    T: TokenRepository,
    U: UserRepository,
    TC: TokenClient,
{
    code_repo: D,
    token_repo: T,
    user_repo: U,
    token_client: TC,
}

impl
    OAuthServiceImpl<CodeRepositoryImpl, TokenRepositoryImpl, UserRepositoryImpl, TokenClientImpl>
{
    pub fn new(
        code_repo: CodeRepositoryImpl,
        token_repo: TokenRepositoryImpl,
        user_repo: UserRepositoryImpl,
        token_client: TokenClientImpl,
    ) -> Self {
        Self {
            code_repo,
            token_repo,
            user_repo,
            token_client,
        }
    }
}

#[crate::instrument_all]
#[async_trait]
impl<D, T, U, TC> OAuthService for OAuthServiceImpl<D, T, U, TC>
where
    D: CodeRepository,
    T: TokenRepository,
    U: UserRepository,
    TC: TokenClient,
{
    async fn request_device_code(
        &self,
        request: DeviceCodeRequest,
    ) -> Result<DeviceCodeResponse, TokenError> {
        let (device_code, device_code_hash) = self.token_client.generate_high_entropic_code();
        let user_code = self.token_client.generate_readable_code();
        let expiry_secs = self.token_client.get_device_code_expiry_in_seconds();
        let expires_at = Utc::now() + Duration::seconds(expiry_secs as i64);

        self.code_repo
            .create_device_authorization(
                &device_code_hash,
                &user_code,
                &request.client_id,
                expires_at,
            )
            .await?;

        Ok(DeviceCodeResponse {
            device_code,
            user_code,
            verification_uri: request.verification_uri,
            expires_in: expiry_secs,
            interval: self.token_client.get_polling_interval_in_seconds(),
        })
    }

    async fn poll_token(&self, request: PollTokenRequest) -> Result<TokenResponse, TokenError> {
        let device_code_hash = hash_string(&request.device_code);
        let device_auth = self
            .code_repo
            .get_device_authorization_by_device_code_hash(&device_code_hash)
            .await?
            .ok_or(TokenError::InvalidDeviceCode)?;

        if device_auth.expires_at < Utc::now()
            && device_auth.status == DeviceAuthorizationStatus::Pending
        {
            self.code_repo
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

                let (access_token, token_hash) = self
                    .token_client
                    .generate_access_token(&TokenType::Personal);

                self.token_repo
                    .create_token(
                        user_id,
                        &device_auth.client_id,
                        &token_hash,
                        TokenType::Personal,
                    )
                    .await?;

                self.code_repo
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
        self.code_repo
            .authorize_device(&request.user_code, request.user_id)
            .await?
            .ok_or(TokenError::InvalidUserCode(
                "User code not found".to_string(),
            ))?;

        Ok(())
    }
}
