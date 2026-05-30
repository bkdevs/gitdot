use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    client::{
        EmailClient, GitHubClient, ImageClient, ImageClientImpl, OctocrabClient, R2Client,
        R2ClientImpl, RedisClient, RedisClientImpl, SmtpClient, TokenClient, TokenClientImpl,
    },
    dto::{
        AuthTokensResponse, ExchangeGitHubCodeRequest, LogoutRequest, OAuthRedirectResponse,
        RefreshSessionRequest, SendAuthEmailRequest, VerifyAuthCodeRequest,
    },
    error::{OptionNotFoundExt, SessionError},
    model::AuthProvider,
    repository::{PgSessionRepository, PgUserRepository, SessionRepository, UserRepository},
    util::{
        auth::{NOREPLY_EMAIL, get_code_email},
        crypto::hash_string,
    },
};

// standard 10 minutes for refresh token grace period
const GRACE_WINDOW: std::time::Duration = std::time::Duration::from_secs(10 * 60);

#[derive(Debug, Serialize, Deserialize)]
struct GraceEntry {
    refresh_token: String,
    expires_at: DateTime<Utc>,
}

/// Owns user login sessions: email magic-code sign-in, GitHub OAuth, refresh
/// token rotation, and logout. Issues short-lived access JWTs paired with
/// rotating refresh tokens organized into per-session families.
#[async_trait]
pub trait SessionService: Send + Sync + 'static {
    /// Emails a one-time login code to `request.email`, creating the user on
    /// first contact.
    ///
    /// If no user exists for the email, one is created (unverified, via the
    /// email provider) and a generated avatar is uploaded best-effort. A
    /// readable auth code is then persisted with the configured expiry and
    /// mailed to the address.
    async fn send_auth_email(&self, request: SendAuthEmailRequest) -> Result<(), SessionError>;

    /// Verifies an emailed login code and, on success, marks the user's email
    /// verified and issues an access/refresh token pair on a fresh session
    /// family. The code is single-use and marked consumed before tokens are
    /// minted. `is_new` reflects whether the email was previously unverified.
    ///
    /// # Errors
    /// - [`SessionError::NotFound`] — no auth code matches
    /// - [`SessionError::TokenRevoked`] — the code was already used
    /// - [`SessionError::TokenExpired`] — the code lapsed before verification
    async fn verify_auth_code(
        &self,
        request: VerifyAuthCodeRequest,
    ) -> Result<AuthTokensResponse, SessionError>;

    /// Rotates a refresh token and returns a new access/refresh pair.
    ///
    /// The previous refresh token enters a 10-minute grace window after a
    /// successful rotation: replays inside the window return the same
    /// replacement tokens issued by the winning rotation (idempotent),
    /// instead of being treated as theft. This absorbs the common case
    /// where parallel browser requests, prefetches, or multi-tab sessions
    /// race the same expired cookie.
    ///
    /// Concurrent rotations of the same token are serialized via a
    /// `SET NX EX` claim in Redis. The first caller does the DB writes;
    /// every other concurrent caller reads the cached replacement and
    /// returns it. The grace entry expires automatically after 10 minutes.
    ///
    /// Reuse detection is preserved: a replay outside the grace window
    /// (or one with no cache entry — e.g., a token revoked by an explicit
    /// `logout`) revokes the entire session family and returns
    /// [`SessionError::TokenRevoked`].
    ///
    /// # Errors
    /// - [`SessionError::NotFound`] — token never existed
    /// - [`SessionError::TokenExpired`] — past the session's `expires_at`
    /// - [`SessionError::TokenRevoked`] — reuse detected; family revoked
    async fn refresh_session(
        &self,
        request: RefreshSessionRequest,
    ) -> Result<AuthTokensResponse, SessionError>;

    /// Revokes the session backing the supplied refresh token, ending it.
    ///
    /// Only the single session is revoked, not its whole family. A subsequent
    /// `refresh_session` with the same token therefore falls outside any grace
    /// window and is treated as reuse.
    ///
    /// # Errors
    /// - [`SessionError::NotFound`] — no session matches the refresh token
    async fn logout(&self, request: LogoutRequest) -> Result<(), SessionError>;

    /// Builds the GitHub OAuth authorization URL, embedding a freshly generated
    /// signed `state` value that is echoed back for CSRF verification during
    /// the code exchange.
    fn get_github_authorization_url(&self) -> OAuthRedirectResponse;

    /// Completes GitHub OAuth: validates `state`, exchanges the code for a
    /// GitHub token, and signs the user in, returning an access/refresh pair.
    ///
    /// Requires a verified primary email on the GitHub account. On first login
    /// the user is created (email pre-verified), and the account's other
    /// verified GitHub emails are upserted as additional verified emails. Each
    /// successful exchange starts a new session family.
    ///
    /// # Errors
    /// - [`SessionError::Unauthorized`] — `state` failed verification, or the
    ///   GitHub primary email is already a verified secondary of another account
    /// - [`SessionError::GitHubError`] — code exchange failed or no verified
    ///   primary email was found
    async fn exchange_github_code(
        &self,
        request: ExchangeGitHubCodeRequest,
    ) -> Result<AuthTokensResponse, SessionError>;
}

#[derive(Debug, Clone)]
pub struct SessionServiceImpl<SR, UR, EC, GH, TC, IC, RC, RD>
where
    SR: SessionRepository,
    UR: UserRepository,
    EC: EmailClient,
    GH: GitHubClient,
    TC: TokenClient,
    IC: ImageClient,
    RC: R2Client,
    RD: RedisClient,
{
    session_repo: SR,
    user_repo: UR,
    email_client: EC,
    github_client: GH,
    token_client: TC,
    image_client: IC,
    r2_client: RC,
    redis_client: RD,
}

impl
    SessionServiceImpl<
        PgSessionRepository,
        PgUserRepository,
        SmtpClient,
        OctocrabClient,
        TokenClientImpl,
        ImageClientImpl,
        R2ClientImpl,
        RedisClientImpl,
    >
{
    pub fn new(
        session_repo: PgSessionRepository,
        user_repo: PgUserRepository,
        email_client: SmtpClient,
        github_client: OctocrabClient,
        token_client: TokenClientImpl,
        image_client: ImageClientImpl,
        r2_client: R2ClientImpl,
        redis_client: RedisClientImpl,
    ) -> Self {
        Self {
            session_repo,
            user_repo,
            email_client,
            github_client,
            token_client,
            image_client,
            r2_client,
            redis_client,
        }
    }
}

impl<SR, UR, EC, GH, TC, IC, RC, RD> SessionServiceImpl<SR, UR, EC, GH, TC, IC, RC, RD>
where
    SR: SessionRepository,
    UR: UserRepository,
    EC: EmailClient,
    GH: GitHubClient,
    TC: TokenClient,
    IC: ImageClient,
    RC: R2Client,
    RD: RedisClient,
{
    fn get_grace_key(&self, old_hash: &str) -> String {
        format!("refresh_grace:{old_hash}")
    }
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl<SR, UR, EC, GH, TC, IC, RC, RD> SessionService
    for SessionServiceImpl<SR, UR, EC, GH, TC, IC, RC, RD>
where
    SR: SessionRepository,
    UR: UserRepository,
    EC: EmailClient,
    GH: GitHubClient,
    TC: TokenClient,
    IC: ImageClient,
    RC: R2Client,
    RD: RedisClient,
{
    async fn send_auth_email(&self, request: SendAuthEmailRequest) -> Result<(), SessionError> {
        let email = request.email.as_ref().to_string();
        let user = match self.user_repo.get_by_primary_email(&email).await? {
            Some(user) => user,
            None => {
                // if the email is taken as a secondary email, do not create a user, just silently fail
                if self.user_repo.is_email_taken(&email).await? {
                    return Ok(());
                }

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

        let (subject, html) = get_code_email(&code);
        self.email_client
            .send_email(NOREPLY_EMAIL, &email, &subject, &html)
            .await?;

        Ok(())
    }

    async fn verify_auth_code(
        &self,
        request: VerifyAuthCodeRequest,
    ) -> Result<AuthTokensResponse, SessionError> {
        let email = request.email.as_ref();
        let user = self
            .user_repo
            .get_by_primary_email(email)
            .await?
            .or_not_found("user", email)?;

        let code = request.code.as_ref();
        let auth_code = self
            .session_repo
            .get_auth_code(user.id, code)
            .await?
            .or_not_found("auth_code", code)?;

        if auth_code.used_at.is_some() {
            return Err(SessionError::TokenRevoked("auth_code".into()));
        }
        if auth_code.expires_at < Utc::now() {
            return Err(SessionError::TokenExpired("auth_code".into()));
        }

        self.session_repo.mark_auth_code_used(auth_code.id).await?;

        self.user_repo.verify_email(user.id).await?;
        let access_token = self.token_client.generate_gitdot_jwt(user.id, &user.name)?;

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

        let is_new = !user.primary_email().is_some_and(|e| e.is_verified);

        Ok(AuthTokensResponse {
            access_token,
            refresh_token,
            access_token_expires_in: self.token_client.get_access_token_expiry_in_seconds(),
            refresh_token_expires_in: refresh_expiry_secs,
            is_new,
        })
    }

    async fn refresh_session(
        &self,
        request: RefreshSessionRequest,
    ) -> Result<AuthTokensResponse, SessionError> {
        let token_hash = hash_string(&request.refresh_token);
        let session = self
            .session_repo
            .get_session_by_refresh_hash(&token_hash)
            .await?
            .or_not_found("session", &token_hash)?;

        if session.expires_at < Utc::now() {
            return Err(SessionError::TokenExpired("session".into()));
        }

        let user = self
            .user_repo
            .get_by_id(session.user_id)
            .await?
            .or_not_found("user", session.user_id)?;
        let access_token = self.token_client.generate_gitdot_jwt(user.id, &user.name)?;
        let access_token_expires_in = self.token_client.get_access_token_expiry_in_seconds();
        let cache_key = self.get_grace_key(&token_hash);

        // Replay path: this token was already rotated. Only honor it inside the grace window.
        if session.revoked_at.is_some() {
            if let Some(grace) = self.redis_client.get::<GraceEntry>(&cache_key).await? {
                let remaining = (grace.expires_at - Utc::now()).num_seconds().max(0) as u64;
                return Ok(AuthTokensResponse {
                    access_token,
                    refresh_token: grace.refresh_token,
                    access_token_expires_in,
                    refresh_token_expires_in: remaining,
                    is_new: false,
                });
            }

            // outside grace, or replacement disappeared → real reuse
            self.session_repo
                .revoke_sessions_by_family(session.refresh_token_family)
                .await?;
            return Err(SessionError::TokenRevoked("session".into()));
        }

        // Happy path: try to claim the rotation for this old token via SET NX.
        let (refresh_token, refresh_token_hash) = self.token_client.generate_high_entropic_code();
        let refresh_expiry_secs = self.token_client.get_refresh_token_expiry_in_seconds();
        let refresh_expiry = Utc::now() + Duration::seconds(refresh_expiry_secs as i64);
        let entry = GraceEntry {
            refresh_token: refresh_token.clone(),
            expires_at: refresh_expiry,
        };
        let claimed = self
            .redis_client
            .set_nx_with_ttl(&cache_key, &entry, GRACE_WINDOW)
            .await?;

        if !claimed {
            // Another worker won the rotation. Read its value and replay.
            match self.redis_client.get::<GraceEntry>(&cache_key).await? {
                Some(existing) => {
                    let remaining = (existing.expires_at - Utc::now()).num_seconds().max(0) as u64;
                    return Ok(AuthTokensResponse {
                        access_token,
                        refresh_token: existing.refresh_token,
                        access_token_expires_in,
                        refresh_token_expires_in: remaining,
                        is_new: false,
                    });
                }
                None => {
                    // Cache vanished between NX-miss and GET — treat as reuse.
                    self.session_repo
                        .revoke_sessions_by_family(session.refresh_token_family)
                        .await?;
                    return Err(SessionError::TokenRevoked("session".into()));
                }
            }
        }

        // We won the claim — persist the rotation.
        self.session_repo.revoke_session(session.id).await?;
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
            access_token_expires_in,
            refresh_token_expires_in: refresh_expiry_secs,
            is_new: false,
        })
    }

    async fn logout(&self, request: LogoutRequest) -> Result<(), SessionError> {
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
    ) -> Result<AuthTokensResponse, SessionError> {
        self.token_client
            .verify_oauth_state(&request.state)
            .map_err(|_| SessionError::Unauthorized)?;

        let github_token = self.github_client.exchange_code(&request.code).await?;
        let github_emails = self.github_client.get_user_emails(&github_token).await?;
        let primary_email = github_emails
            .iter()
            .find(|e| e.primary && e.verified)
            .map(|e| e.email.clone())
            .ok_or_else(|| {
                SessionError::GitHubError(crate::error::GitHubError::Other(
                    "No verified primary email found".to_string(),
                ))
            })?;
        let (user, is_new) = match self.user_repo.get_by_primary_email(&primary_email).await? {
            Some(user) => (user, false),
            None => {
                // if the email is taken as a secondary email, throw an unauthorized error in oauth
                if self.user_repo.is_email_taken(&primary_email).await? {
                    return Err(SessionError::Unauthorized);
                }

                let user = self
                    .user_repo
                    .create(&primary_email, true, AuthProvider::GitHub)
                    .await?;
                (user, true)
            }
        };

        if is_new {
            let extras: Vec<String> = github_emails
                .into_iter()
                .filter(|e| e.verified && e.email != primary_email)
                .map(|e| e.email)
                .collect();
            if !extras.is_empty() {
                self.user_repo
                    .upsert_verified_emails(user.id, &extras)
                    .await?;
            }
        }

        let access_token = self.token_client.generate_gitdot_jwt(user.id, &user.name)?;

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
}
