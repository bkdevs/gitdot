use async_trait::async_trait;
use chrono::{Duration, Utc};

use crate::{
    client::{TokenClient, TokenClientImpl},
    dto::{
        AuthorizeDeviceRequest, DeviceCodeRequest, DeviceCodeResponse, PollTokenRequest,
        TokenResponse,
    },
    error::{DeviceError, InputError, OptionNotFoundExt},
    model::{DeviceAuthorizationStatus, TokenType},
    repository::{
        DeviceRepository, PgDeviceRepository, PgTokenRepository, PgUserRepository, TokenRepository,
        UserRepository,
    },
    util::crypto::hash_string,
};

/// Implements the OAuth 2.0 device authorization flow. A client requests a
/// device/user code pair, the user approves the user code in a browser, and
/// the client polls until it can exchange the device code for an access token.
#[async_trait]
pub trait DeviceService: Send + Sync + 'static {
    /// Starts a device authorization: generates a device code (returned in the
    /// clear, stored only as a hash), a human-readable user code, and persists
    /// a pending authorization that expires after the configured device-code
    /// window. Returns the codes alongside the verification URL, expiry, and
    /// the recommended polling interval.
    async fn request_device_code(
        &self,
        request: DeviceCodeRequest,
    ) -> Result<DeviceCodeResponse, DeviceError>;

    /// Polls the status of a pending authorization by device code and, once
    /// authorized, issues a personal access token for the approving user.
    ///
    /// A successful authorization consumes the request: the device
    /// authorization is expired after the token is created, so the token can
    /// be retrieved only once.
    ///
    /// # Errors
    /// - [`DeviceError::NotFound`] — no authorization matches the device code
    /// - [`DeviceError::TokenPending`] — user has not yet approved the user code
    /// - [`DeviceError::TokenExpired`] — the device code lapsed before approval
    async fn poll_token(&self, request: PollTokenRequest) -> Result<TokenResponse, DeviceError>;

    /// Approves a pending authorization by binding the user code to the
    /// approving user, unblocking the client's next `poll_token` call.
    ///
    /// # Errors
    /// - [`DeviceError::NotFound`] — no pending authorization matches the user code
    async fn authorize_device(&self, request: AuthorizeDeviceRequest) -> Result<(), DeviceError>;
}

#[derive(Debug, Clone)]
pub struct DeviceServiceImpl<DR, TR, UR, TC>
where
    DR: DeviceRepository,
    TR: TokenRepository,
    UR: UserRepository,
    TC: TokenClient,
{
    device_repo: DR,
    token_repo: TR,
    user_repo: UR,
    token_client: TC,
}

impl DeviceServiceImpl<PgDeviceRepository, PgTokenRepository, PgUserRepository, TokenClientImpl> {
    pub fn new(
        device_repo: PgDeviceRepository,
        token_repo: PgTokenRepository,
        user_repo: PgUserRepository,
        token_client: TokenClientImpl,
    ) -> Self {
        Self {
            device_repo,
            token_repo,
            user_repo,
            token_client,
        }
    }
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl<DR, TR, UR, TC> DeviceService for DeviceServiceImpl<DR, TR, UR, TC>
where
    DR: DeviceRepository,
    TR: TokenRepository,
    UR: UserRepository,
    TC: TokenClient,
{
    async fn request_device_code(
        &self,
        request: DeviceCodeRequest,
    ) -> Result<DeviceCodeResponse, DeviceError> {
        let (device_code, device_code_hash) = self.token_client.generate_high_entropic_code();
        let (user_code, user_code_hash) = self.token_client.generate_readable_code();
        let expiry_secs = self.token_client.get_device_code_expiry_in_seconds();
        let expires_at = Utc::now() + Duration::seconds(expiry_secs as i64);

        self.device_repo
            .create_device_authorization(
                &device_code_hash,
                &user_code_hash,
                &request.client_id,
                expires_at,
            )
            .await?;

        Ok(DeviceCodeResponse {
            device_code,
            user_code,
            verification_url: request.verification_url,
            expires_in: expiry_secs,
            interval: self.token_client.get_polling_interval_in_seconds(),
        })
    }

    async fn poll_token(&self, request: PollTokenRequest) -> Result<TokenResponse, DeviceError> {
        let device_code_hash = hash_string(&request.device_code);
        let device_auth = self
            .device_repo
            .get_device_authorization_by_device_code_hash(&device_code_hash)
            .await?
            .or_not_found("device_code", &device_code_hash)?;

        if device_auth.expires_at < Utc::now()
            && device_auth.status == DeviceAuthorizationStatus::Pending
        {
            self.device_repo
                .expire_device_authorization(device_auth.id)
                .await?;
            return Err(DeviceError::TokenExpired("device_code".into()));
        }

        match device_auth.status {
            DeviceAuthorizationStatus::Pending => {
                Err(DeviceError::TokenPending("device_code".into()))
            }
            DeviceAuthorizationStatus::Expired => {
                Err(DeviceError::TokenExpired("device_code".into()))
            }
            DeviceAuthorizationStatus::Authorized => {
                let user_id = device_auth
                    .user_id
                    .ok_or(InputError::new("user_id", "missing"))?;

                let user = self
                    .user_repo
                    .get_by_id(user_id)
                    .await?
                    .or_not_found("user", user_id)?;
                let user_email = user
                    .primary_email()
                    .or_not_found("user_email", user_id)?
                    .email
                    .clone();

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

                self.device_repo
                    .expire_device_authorization(device_auth.id)
                    .await?;

                Ok(TokenResponse {
                    access_token,
                    user_name: user.name,
                    user_email,
                })
            }
        }
    }

    async fn authorize_device(&self, request: AuthorizeDeviceRequest) -> Result<(), DeviceError> {
        let user_code_hash = hash_string(request.user_code.as_ref());
        self.device_repo
            .authorize_device(&user_code_hash, request.user_id)
            .await?
            .or_not_found("user_code", &user_code_hash)?;

        Ok(())
    }
}
