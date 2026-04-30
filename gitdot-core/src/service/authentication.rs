use async_trait::async_trait;
use chrono::{Duration, Utc};

use crate::{
    client::{
        EmailClient, GitHubClient, ImageClient, ImageClientImpl, OctocrabClient, R2Client,
        R2ClientImpl, ResendClient, TokenClient, TokenClientImpl,
    },
    dto::{
        AuthTokensResponse, AuthorizeDeviceRequest, DeviceCodeRequest, DeviceCodeResponse,
        ExchangeGitHubCodeRequest, IssueTaskJwtRequest, IssueTaskJwtResponse, JwtClaims,
        LinkSlackAccountRequest, LinkSlackAccountResponse, LogoutRequest, OAuthRedirectResponse,
        PollTokenRequest, RefreshSessionRequest, SendAuthEmailRequest, TokenResponse,
        ValidateTokenRequest, ValidateTokenResponse, VerifyAuthCodeRequest,
    },
    error::{AuthenticationError, InputError, OptionNotFoundExt},
    model::{AuthProvider, DeviceAuthorizationStatus, TokenType},
    repository::{
        DeviceRepository, DeviceRepositoryImpl, SessionRepository, SessionRepositoryImpl,
        SlackRepository, SlackRepositoryImpl, TokenRepository, TokenRepositoryImpl, UserRepository,
        UserRepositoryImpl,
    },
    util::{
        auth::{GITDOT_SERVER_ID, NOREPLY_EMAIL, S2_SERVER_ID, get_auth_email},
        crypto::hash_string,
    },
};

#[async_trait]
pub trait AuthenticationService: Send + Sync + 'static {
    // --- Auth operations ---

    async fn send_auth_email(
        &self,
        request: SendAuthEmailRequest,
    ) -> Result<(), AuthenticationError>;

    async fn verify_auth_code(
        &self,
        request: VerifyAuthCodeRequest,
    ) -> Result<AuthTokensResponse, AuthenticationError>;

    async fn refresh_session(
        &self,
        request: RefreshSessionRequest,
    ) -> Result<AuthTokensResponse, AuthenticationError>;

    async fn logout(&self, request: LogoutRequest) -> Result<(), AuthenticationError>;

    // --- OAuth operations ---

    fn get_github_authorization_url(&self) -> OAuthRedirectResponse;

    async fn exchange_github_code(
        &self,
        request: ExchangeGitHubCodeRequest,
    ) -> Result<AuthTokensResponse, AuthenticationError>;

    async fn link_slack_account(
        &self,
        request: LinkSlackAccountRequest,
    ) -> Result<LinkSlackAccountResponse, AuthenticationError>;

    // --- Device flow operations ---

    async fn request_device_code(
        &self,
        request: DeviceCodeRequest,
    ) -> Result<DeviceCodeResponse, AuthenticationError>;

    async fn poll_token(
        &self,
        request: PollTokenRequest,
    ) -> Result<TokenResponse, AuthenticationError>;

    async fn authorize_device(
        &self,
        request: AuthorizeDeviceRequest,
    ) -> Result<(), AuthenticationError>;

    // --- Token operations ---

    async fn validate_token(
        &self,
        request: ValidateTokenRequest,
    ) -> Result<ValidateTokenResponse, AuthenticationError>;

    async fn issue_task_token(
        &self,
        request: IssueTaskJwtRequest,
    ) -> Result<IssueTaskJwtResponse, AuthenticationError>;
}

#[derive(Debug, Clone)]
pub struct AuthenticationServiceImpl<DR, SR, SlR, TR, UR, EC, GH, TC, IC, RC>
where
    DR: DeviceRepository,
    SR: SessionRepository,
    SlR: SlackRepository,
    TR: TokenRepository,
    UR: UserRepository,
    EC: EmailClient,
    GH: GitHubClient,
    TC: TokenClient,
    IC: ImageClient,
    RC: R2Client,
{
    device_repo: DR,
    session_repo: SR,
    slack_repo: SlR,
    token_repo: TR,
    user_repo: UR,
    email_client: EC,
    github_client: GH,
    token_client: TC,
    image_client: IC,
    r2_client: RC,
}

impl
    AuthenticationServiceImpl<
        DeviceRepositoryImpl,
        SessionRepositoryImpl,
        SlackRepositoryImpl,
        TokenRepositoryImpl,
        UserRepositoryImpl,
        ResendClient,
        OctocrabClient,
        TokenClientImpl,
        ImageClientImpl,
        R2ClientImpl,
    >
{
    pub fn new(
        device_repo: DeviceRepositoryImpl,
        session_repo: SessionRepositoryImpl,
        slack_repo: SlackRepositoryImpl,
        token_repo: TokenRepositoryImpl,
        user_repo: UserRepositoryImpl,
        email_client: ResendClient,
        github_client: OctocrabClient,
        token_client: TokenClientImpl,
        image_client: ImageClientImpl,
        r2_client: R2ClientImpl,
    ) -> Self {
        Self {
            device_repo,
            session_repo,
            slack_repo,
            token_repo,
            user_repo,
            email_client,
            github_client,
            token_client,
            image_client,
            r2_client,
        }
    }
}

#[crate::instrument_all]
#[async_trait]
impl<DR, SR, SlR, TR, UR, EC, GH, TC, IC, RC> AuthenticationService
    for AuthenticationServiceImpl<DR, SR, SlR, TR, UR, EC, GH, TC, IC, RC>
where
    DR: DeviceRepository,
    SR: SessionRepository,
    SlR: SlackRepository,
    TR: TokenRepository,
    UR: UserRepository,
    EC: EmailClient,
    GH: GitHubClient,
    TC: TokenClient,
    IC: ImageClient,
    RC: R2Client,
{
    async fn send_auth_email(
        &self,
        request: SendAuthEmailRequest,
    ) -> Result<(), AuthenticationError> {
        let email = request.email.as_ref().to_string();
        let user = match self.user_repo.get_by_email(&email).await? {
            Some(user) => user,
            None => {
                let user = self
                    .user_repo
                    .create(&email, false, AuthProvider::Email)
                    .await?;

                if let Ok(image_bytes) = self.image_client.generate_user_image(&email).await {
                    self.r2_client
                        .upload_object(&format!("users/{}.webp", user.id), image_bytes)
                        .await
                        .ok();
                }
                user
            }
        };

        let code = self.token_client.generate_readable_code();
        let expiry_secs = self.token_client.get_auth_code_expiry_in_seconds();
        let expires_at = Utc::now() + Duration::seconds(expiry_secs as i64);
        self.session_repo
            .create_auth_code(user.id, &code, expires_at)
            .await?;

        let (subject, html) = get_auth_email(&code);
        self.email_client
            .send_email(NOREPLY_EMAIL, &email, &subject, &html)
            .await?;

        Ok(())
    }

    async fn verify_auth_code(
        &self,
        request: VerifyAuthCodeRequest,
    ) -> Result<AuthTokensResponse, AuthenticationError> {
        let auth_code = self
            .session_repo
            .get_auth_code(&request.code)
            .await?
            .or_not_found("auth_code", &request.code)?;

        if auth_code.used_at.is_some() {
            return Err(AuthenticationError::TokenRevoked("auth_code".into()));
        }
        if auth_code.expires_at < Utc::now() {
            return Err(AuthenticationError::TokenExpired("auth_code".into()));
        }

        self.session_repo.mark_auth_code_used(auth_code.id).await?;
        self.user_repo.verify_email(auth_code.user_id).await?;

        let user = self
            .user_repo
            .get_by_id(auth_code.user_id)
            .await?
            .or_not_found("user", auth_code.user_id)?;
        let orgs = self
            .user_repo
            .get_org_memberships(auth_code.user_id)
            .await?;
        let access_token = self
            .token_client
            .generate_gitdot_jwt(user.id, &user.name, &orgs)?;

        let (refresh_token, refresh_token_hash) = self.token_client.generate_high_entropic_code();
        let refresh_expiry_secs = self.token_client.get_refresh_token_expiry_in_seconds();
        let refresh_expiry = Utc::now() + Duration::seconds(refresh_expiry_secs as i64);
        self.session_repo
            .create_session(
                auth_code.user_id,
                &refresh_token_hash,
                uuid::Uuid::new_v4(),
                request.user_agent.as_deref(),
                request.ip_address,
                refresh_expiry,
            )
            .await?;

        Ok(AuthTokensResponse {
            access_token,
            refresh_token,
            access_token_expires_in: self.token_client.get_access_token_expiry_in_seconds(),
            refresh_token_expires_in: refresh_expiry_secs,
            is_new: false,
        })
    }

    async fn refresh_session(
        &self,
        request: RefreshSessionRequest,
    ) -> Result<AuthTokensResponse, AuthenticationError> {
        let token_hash = hash_string(&request.refresh_token);
        let session = self
            .session_repo
            .get_session_by_refresh_hash(&token_hash)
            .await?
            .or_not_found("session", &token_hash)?;

        if session.revoked_at.is_some() {
            self.session_repo
                .revoke_sessions_by_family(session.refresh_token_family)
                .await?;
            return Err(AuthenticationError::TokenRevoked("session".into()));
        }
        if session.expires_at < Utc::now() {
            return Err(AuthenticationError::TokenExpired("session".into()));
        }

        self.session_repo.revoke_session(session.id).await?;

        let user = self
            .user_repo
            .get_by_id(session.user_id)
            .await?
            .or_not_found("user", session.user_id)?;
        let orgs = self.user_repo.get_org_memberships(session.user_id).await?;
        let access_token = self
            .token_client
            .generate_gitdot_jwt(user.id, &user.name, &orgs)?;

        let (refresh_token, refresh_token_hash) = self.token_client.generate_high_entropic_code();
        let refresh_expiry_secs = self.token_client.get_refresh_token_expiry_in_seconds();
        let refresh_expiry = Utc::now() + Duration::seconds(refresh_expiry_secs as i64);
        self.session_repo
            .create_session(
                session.user_id,
                &refresh_token_hash,
                session.refresh_token_family,
                request.user_agent.as_deref(),
                request.ip_address,
                refresh_expiry,
            )
            .await?;

        Ok(AuthTokensResponse {
            access_token,
            refresh_token,
            access_token_expires_in: self.token_client.get_access_token_expiry_in_seconds(),
            refresh_token_expires_in: refresh_expiry_secs,
            is_new: false,
        })
    }

    async fn logout(&self, request: LogoutRequest) -> Result<(), AuthenticationError> {
        let token_hash = hash_string(&request.refresh_token);
        let session = self
            .session_repo
            .get_session_by_refresh_hash(&token_hash)
            .await?
            .or_not_found("session", &token_hash)?;

        self.session_repo.revoke_session(session.id).await?;

        Ok(())
    }

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
            .map_err(|_| AuthenticationError::Unauthorized)?;

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
            .generate_gitdot_jwt(user.id, &user.name, &orgs)?;

        let (refresh_token, refresh_token_hash) = self.token_client.generate_high_entropic_code();
        let refresh_expiry_secs = self.token_client.get_refresh_token_expiry_in_seconds();
        let refresh_expiry = Utc::now() + Duration::seconds(refresh_expiry_secs as i64);
        self.session_repo
            .create_session(
                user.id,
                &refresh_token_hash,
                uuid::Uuid::new_v4(),
                request.user_agent.as_deref(),
                request.ip_address,
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

    async fn link_slack_account(
        &self,
        request: LinkSlackAccountRequest,
    ) -> Result<LinkSlackAccountResponse, AuthenticationError> {
        let payload = self
            .token_client
            .verify_slack_state(&request.state)
            .map_err(|_| AuthenticationError::Unauthorized)?;

        let account = self
            .slack_repo
            .create_slack_account(
                request.gitdot_user_id,
                &payload.slack_user_id,
                &payload.team_id,
            )
            .await?;

        Ok(account.into())
    }

    async fn request_device_code(
        &self,
        request: DeviceCodeRequest,
    ) -> Result<DeviceCodeResponse, AuthenticationError> {
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

    async fn poll_token(
        &self,
        request: PollTokenRequest,
    ) -> Result<TokenResponse, AuthenticationError> {
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
            return Err(AuthenticationError::TokenExpired("device_code".into()));
        }

        match device_auth.status {
            DeviceAuthorizationStatus::Pending => {
                return Err(AuthenticationError::TokenPending("device_code".into()));
            }
            DeviceAuthorizationStatus::Expired => {
                return Err(AuthenticationError::TokenExpired("device_code".into()));
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

    async fn authorize_device(
        &self,
        request: AuthorizeDeviceRequest,
    ) -> Result<(), AuthenticationError> {
        self.device_repo
            .authorize_device(&request.user_code, request.user_id)
            .await?
            .or_not_found("user_code", request.user_code.as_ref())?;

        Ok(())
    }

    async fn validate_token(
        &self,
        request: ValidateTokenRequest,
    ) -> Result<ValidateTokenResponse, AuthenticationError> {
        if !self.token_client.validate_token_format(&request.token) {
            return Err(AuthenticationError::Unauthorized);
        }
        if !&request.token.starts_with(request.token_type.prefix()) {
            return Err(AuthenticationError::Unauthorized);
        }

        let token_hash = hash_string(&request.token);
        let access_token = self
            .token_repo
            .get_token_by_hash(&token_hash)
            .await?
            .ok_or(AuthenticationError::Unauthorized)?;

        self.token_repo.touch_token(access_token.id).await?;

        Ok(ValidateTokenResponse {
            principal_id: access_token.principal_id,
        })
    }

    async fn issue_task_token(
        &self,
        request: IssueTaskJwtRequest,
    ) -> Result<IssueTaskJwtResponse, AuthenticationError> {
        let now = Utc::now().timestamp() as usize;
        let claims = JwtClaims {
            iss: GITDOT_SERVER_ID.to_string(),
            aud: vec![GITDOT_SERVER_ID.to_string(), S2_SERVER_ID.to_string()],
            sub: request.task_id.to_string(),
            iat: now,
            exp: now + request.duration.as_secs() as usize,
        };
        let token = self.token_client.generate_jwt(&claims)?;

        Ok(IssueTaskJwtResponse { token })
    }
}
