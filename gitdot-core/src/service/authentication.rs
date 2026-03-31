use async_trait::async_trait;
use chrono::{Duration, Utc};

use crate::{
    client::{EmailClient, ResendClient, TokenClient, TokenClientImpl},
    dto::{
        AuthTokensResponse, IssueTaskJwtRequest, IssueTaskJwtResponse, JwtClaims, LogoutRequest,
        RefreshSessionRequest, SendAuthEmailRequest, ValidateTokenRequest, ValidateTokenResponse,
        VerifyAuthCodeRequest,
    },
    error::{AuthenticationError, AuthorizationError},
    repository::{
        SessionRepository, SessionRepositoryImpl, TokenRepository, TokenRepositoryImpl,
        UserRepository, UserRepositoryImpl,
    },
    util::{
        auth::{GITDOT_SERVER_ID, NOREPLY_EMAIL, S2_SERVER_ID, get_auth_email},
        crypto::hash_string,
    },
};

#[async_trait]
pub trait AuthenticationService: Send + Sync + 'static {
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

    async fn validate_token(
        &self,
        request: ValidateTokenRequest,
    ) -> Result<ValidateTokenResponse, AuthorizationError>;

    async fn issue_task_token(
        &self,
        request: IssueTaskJwtRequest,
    ) -> Result<IssueTaskJwtResponse, AuthorizationError>;
}

#[derive(Debug, Clone)]
pub struct AuthenticationServiceImpl<SR, TR, UR, EC, TC>
where
    SR: SessionRepository,
    TR: TokenRepository,
    UR: UserRepository,
    EC: EmailClient,
    TC: TokenClient,
{
    session_repo: SR,
    token_repo: TR,
    user_repo: UR,
    email_client: EC,
    token_client: TC,
}

impl
    AuthenticationServiceImpl<
        SessionRepositoryImpl,
        TokenRepositoryImpl,
        UserRepositoryImpl,
        ResendClient,
        TokenClientImpl,
    >
{
    pub fn new(
        session_repo: SessionRepositoryImpl,
        token_repo: TokenRepositoryImpl,
        user_repo: UserRepositoryImpl,
        email_client: ResendClient,
        token_client: TokenClientImpl,
    ) -> Self {
        Self {
            session_repo,
            token_repo,
            user_repo,
            email_client,
            token_client,
        }
    }
}

#[crate::instrument_all]
#[async_trait]
impl<SR, TR, UR, EC, TC> AuthenticationService for AuthenticationServiceImpl<SR, TR, UR, EC, TC>
where
    SR: SessionRepository,
    TR: TokenRepository,
    UR: UserRepository,
    EC: EmailClient,
    TC: TokenClient,
{
    async fn send_auth_email(
        &self,
        request: SendAuthEmailRequest,
    ) -> Result<(), AuthenticationError> {
        let email = request.email.as_ref().to_string();
        let (user, is_signup) = match self.user_repo.get_by_email(&email).await? {
            Some(user) => (user, false),
            None => {
                let user = self.user_repo.create(&email).await?;
                (user, true)
            }
        };

        let (code, code_hash) = self.token_client.generate_high_entropic_code();
        let expiry_secs = self.token_client.get_auth_code_expiry_in_seconds();
        let expires_at = Utc::now() + Duration::seconds(expiry_secs as i64);
        self.session_repo
            .create_auth_code(user.id, &code_hash, expires_at)
            .await?;

        let (subject, html) = get_auth_email(is_signup, &code);
        self.email_client
            .send_email(NOREPLY_EMAIL, &email, &subject, &html)
            .await
            .map_err(|e| AuthenticationError::EmailError(e.to_string()))?;

        Ok(())
    }

    async fn verify_auth_code(
        &self,
        request: VerifyAuthCodeRequest,
    ) -> Result<AuthTokensResponse, AuthenticationError> {
        let code_hash = hash_string(&request.code);
        let auth_code = self
            .session_repo
            .get_auth_code_by_hash(&code_hash)
            .await?
            .ok_or(AuthenticationError::AuthCodeNotFound)?;

        if auth_code.used_at.is_some() {
            return Err(AuthenticationError::AuthCodeAlreadyUsed);
        }
        if auth_code.expires_at < Utc::now() {
            return Err(AuthenticationError::AuthCodeExpired);
        }

        self.session_repo.mark_auth_code_used(auth_code.id).await?;
        self.user_repo.verify_email(auth_code.user_id).await?;

        let user = self
            .user_repo
            .get_by_id(auth_code.user_id)
            .await?
            .ok_or(AuthenticationError::AuthCodeNotFound)?;
        let orgs = self
            .user_repo
            .get_org_memberships(auth_code.user_id)
            .await?;
        let access_token = self
            .token_client
            .generate_gitdot_jwt(user.id, &user.name, &orgs)
            .map_err(AuthenticationError::JwtError)?;

        let (refresh_token, refresh_token_hash) = self.token_client.generate_high_entropic_code();
        let refresh_expiry_secs = self.token_client.get_refresh_token_expiry_in_seconds();
        let refresh_expiry = Utc::now() + Duration::seconds(refresh_expiry_secs as i64);
        self.session_repo
            .create_session(
                auth_code.user_id,
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
            .ok_or(AuthenticationError::SessionNotFound)?;

        if session.revoked_at.is_some() {
            self.session_repo
                .revoke_sessions_by_family(session.refresh_token_family)
                .await?;
            return Err(AuthenticationError::SessionRevoked);
        }
        if session.expires_at < Utc::now() {
            return Err(AuthenticationError::SessionExpired);
        }

        self.session_repo.revoke_session(session.id).await?;

        let user = self
            .user_repo
            .get_by_id(session.user_id)
            .await?
            .ok_or(AuthenticationError::SessionNotFound)?;
        let orgs = self.user_repo.get_org_memberships(session.user_id).await?;
        let access_token = self
            .token_client
            .generate_gitdot_jwt(user.id, &user.name, &orgs)
            .map_err(AuthenticationError::JwtError)?;

        let (refresh_token, refresh_token_hash) = self.token_client.generate_high_entropic_code();
        let refresh_expiry_secs = self.token_client.get_refresh_token_expiry_in_seconds();
        let refresh_expiry = Utc::now() + Duration::seconds(refresh_expiry_secs as i64);
        self.session_repo
            .create_session(
                session.user_id,
                &refresh_token_hash,
                session.refresh_token_family,
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
        })
    }

    async fn logout(&self, request: LogoutRequest) -> Result<(), AuthenticationError> {
        let token_hash = hash_string(&request.refresh_token);
        let session = self
            .session_repo
            .get_session_by_refresh_hash(&token_hash)
            .await?
            .ok_or(AuthenticationError::SessionNotFound)?;

        self.session_repo.revoke_session(session.id).await?;

        Ok(())
    }

    async fn validate_token(
        &self,
        request: ValidateTokenRequest,
    ) -> Result<ValidateTokenResponse, AuthorizationError> {
        if !self.token_client.validate_token_format(&request.token) {
            return Err(AuthorizationError::Unauthorized);
        }
        if !&request.token.starts_with(request.token_type.prefix()) {
            return Err(AuthorizationError::Unauthorized);
        }

        let token_hash = hash_string(&request.token);
        let access_token = self
            .token_repo
            .get_token_by_hash(&token_hash)
            .await?
            .ok_or(AuthorizationError::Unauthorized)?;

        self.token_repo.touch_token(access_token.id).await?;

        Ok(ValidateTokenResponse {
            principal_id: access_token.principal_id,
        })
    }

    async fn issue_task_token(
        &self,
        request: IssueTaskJwtRequest,
    ) -> Result<IssueTaskJwtResponse, AuthorizationError> {
        let now = Utc::now().timestamp() as usize;
        let claims = JwtClaims {
            iss: GITDOT_SERVER_ID.to_string(),
            aud: vec![GITDOT_SERVER_ID.to_string(), S2_SERVER_ID.to_string()],
            sub: request.task_id.to_string(),
            iat: now,
            exp: now + request.duration.as_secs() as usize,
        };
        let token = self
            .token_client
            .generate_jwt(&claims)
            .map_err(AuthorizationError::InvalidToken)?;

        Ok(IssueTaskJwtResponse { token })
    }
}
