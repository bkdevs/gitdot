use async_trait::async_trait;
use chrono::{Duration, Utc};

use crate::{
    client::{GitHubClient, OctocrabClient, TokenClient, TokenClientImpl},
    dto::{
        AuthTokensResponse, AuthorizeDeviceRequest, DeviceCodeRequest, DeviceCodeResponse,
        ExchangeGitHubCodeRequest, OAuthRedirectResponse, PollTokenRequest, TokenResponse,
    },
    error::{AuthenticationError, TokenError},
    model::{AuthProvider, DeviceAuthorizationStatus, TokenType},
    repository::{
        DeviceRepository, DeviceRepositoryImpl, SessionRepository, SessionRepositoryImpl,
        TokenRepository, TokenRepositoryImpl, UserRepository, UserRepositoryImpl,
    },
    util::crypto::hash_string,
};

#[async_trait]
pub trait OAuthService: Send + Sync + 'static {
    // GitHub OAuth operations
    fn get_github_authorization_url(&self) -> OAuthRedirectResponse;
    async fn exchange_github_code(
        &self,
        request: ExchangeGitHubCodeRequest,
    ) -> Result<AuthTokensResponse, AuthenticationError>;

    // Device flow operations
    async fn request_device_code(
        &self,
        request: DeviceCodeRequest,
    ) -> Result<DeviceCodeResponse, TokenError>;
    async fn poll_token(&self, request: PollTokenRequest) -> Result<TokenResponse, TokenError>;
    async fn authorize_device(&self, request: AuthorizeDeviceRequest) -> Result<(), TokenError>;
}

#[derive(Debug, Clone)]
pub struct OAuthServiceImpl<D, S, T, U, GH, TC>
where
    D: DeviceRepository,
    S: SessionRepository,
    T: TokenRepository,
    U: UserRepository,
    GH: GitHubClient,
    TC: TokenClient,
{
    device_repo: D,
    session_repo: S,
    token_repo: T,
    user_repo: U,
    github_client: GH,
    token_client: TC,
}

impl
    OAuthServiceImpl<
        DeviceRepositoryImpl,
        SessionRepositoryImpl,
        TokenRepositoryImpl,
        UserRepositoryImpl,
        OctocrabClient,
        TokenClientImpl,
    >
{
    pub fn new(
        device_repo: DeviceRepositoryImpl,
        session_repo: SessionRepositoryImpl,
        token_repo: TokenRepositoryImpl,
        user_repo: UserRepositoryImpl,
        github_client: OctocrabClient,
        token_client: TokenClientImpl,
    ) -> Self {
        Self {
            device_repo,
            session_repo,
            token_repo,
            user_repo,
            github_client,
            token_client,
        }
    }
}

#[crate::instrument_all]
#[async_trait]
impl<D, S, T, U, GH, TC> OAuthService for OAuthServiceImpl<D, S, T, U, GH, TC>
where
    D: DeviceRepository,
    S: SessionRepository,
    T: TokenRepository,
    U: UserRepository,
    GH: GitHubClient,
    TC: TokenClient,
{
    fn get_github_authorization_url(&self) -> OAuthRedirectResponse {
        let state = self.token_client.generate_oauth_state();
        let authorize_url = self.github_client.get_authorization_url(&state);
        OAuthRedirectResponse {
            authorize_url,
            state,
        }
    }

    async fn exchange_github_code(
        &self,
        request: ExchangeGitHubCodeRequest,
    ) -> Result<AuthTokensResponse, AuthenticationError> {
        self.token_client
            .verify_oauth_state(&request.state)
            .map_err(AuthenticationError::InvalidOAuthState)?;

        let github_token = self.github_client.exchange_code(&request.code).await?;
        let email = self.github_client.get_user_email(&github_token).await?;
        let (user, is_new) = match self.user_repo.get_by_email(&email).await? {
            Some(user) => (user, false),
            None => {
                let user = self
                    .user_repo
                    .create(&email, true, AuthProvider::GitHub)
                    .await?;
                (user, true)
            }
        };

        let orgs = self.user_repo.get_org_memberships(user.id).await?;
        let access_token = self
            .token_client
            .generate_gitdot_jwt(user.id, &user.name, &orgs)
            .map_err(AuthenticationError::JwtError)?;

        let (refresh_token, refresh_token_hash) = self.token_client.generate_high_entropic_code();
        let refresh_expiry_secs = self.token_client.get_refresh_token_expiry_in_seconds();
        let refresh_expiry = Utc::now() + Duration::seconds(refresh_expiry_secs as i64);
        self.session_repo
            .create_session(
                user.id,
                &refresh_token_hash,
                uuid::Uuid::new_v4(),
                request.user_agent.as_deref(),
                request.ip_address.as_ref().map(|ip| ip.as_ref()),
                refresh_expiry,
            )
            .await?;

        Ok(AuthTokensResponse {
            access_token,
            refresh_token,
            access_token_expires_in: self.token_client.get_access_token_expiry_in_seconds(),
            refresh_token_expires_in: refresh_expiry_secs,
            is_new,
        })
    }

    async fn request_device_code(
        &self,
        request: DeviceCodeRequest,
    ) -> Result<DeviceCodeResponse, TokenError> {
        let (device_code, device_code_hash) = self.token_client.generate_high_entropic_code();
        let user_code = self.token_client.generate_readable_code();
        let expiry_secs = self.token_client.get_device_code_expiry_in_seconds();
        let expires_at = Utc::now() + Duration::seconds(expiry_secs as i64);

        self.device_repo
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
            .device_repo
            .get_device_authorization_by_device_code_hash(&device_code_hash)
            .await?
            .ok_or(TokenError::InvalidDeviceCode)?;

        if device_auth.expires_at < Utc::now()
            && device_auth.status == DeviceAuthorizationStatus::Pending
        {
            self.device_repo
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

                self.device_repo
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
        self.device_repo
            .authorize_device(&request.user_code, request.user_id)
            .await?
            .ok_or(TokenError::InvalidUserCode(
                "User code not found".to_string(),
            ))?;

        Ok(())
    }
}
