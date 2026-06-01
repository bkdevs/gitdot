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
    repository::{
        AuthCodeVerification, PgSessionRepository, PgUserRepository, SessionRepository,
        UserRepository,
    },
    util::{
        auth::{NOREPLY_EMAIL, get_code_email},
        crypto::hash_string,
    },
};

// standard 10 minutes for refresh token grace period
const GRACE_WINDOW: std::time::Duration = std::time::Duration::from_secs(10 * 60);

// number of wrong login attempts allowed before the active code is invalidated
const MAX_AUTH_CODE_ATTEMPTS: i16 = 5;

// minimum time between login-code sends to a single email
const AUTH_CODE_SEND_COOLDOWN: std::time::Duration = std::time::Duration::from_secs(30);

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
    /// family. The code is single-use and consumed before tokens are minted, and
    /// is burned after [`MAX_AUTH_CODE_ATTEMPTS`] wrong guesses. `is_new`
    /// reflects whether the email was previously unverified.
    ///
    /// # Errors
    /// - [`SessionError::Unauthorized`] — returned for every failure mode
    ///   (unknown account, wrong code, expired, already used, or locked out),
    ///   deliberately undifferentiated to avoid an enumeration/probing oracle.
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
    fn get_rate_limit_key(&self, email: &str) -> String {
        format!("auth_code_send:{}", hash_string(&email))
    }

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

        // rate-limit code sends per email
        let rate_limit_key = self.get_rate_limit_key(&email);
        match self
            .redis_client
            .set_nx_with_ttl(&rate_limit_key, &true, AUTH_CODE_SEND_COOLDOWN)
            .await
        {
            Ok(true) => {}
            Ok(false) => return Err(SessionError::TooManyAttempts),
            Err(e) => {
                tracing::warn!(error = %e, "auth code send rate-limit check failed; allowing send")
            }
        }

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

        let (code, code_hash) = self.token_client.generate_readable_code();
        let expiry_secs = self.token_client.get_auth_code_expiry_in_seconds();
        let expires_at = Utc::now() + Duration::seconds(expiry_secs as i64);

        // invalidate any prior active code so exactly one code is verifiable at a time
        self.session_repo.invalidate_auth_codes(user.id).await?;
        self.session_repo
            .create_auth_code(user.id, &code_hash, expires_at)
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
        let Some(user) = self.user_repo.get_by_primary_email(email).await? else {
            return Err(SessionError::Unauthorized);
        };

        let code_hash = hash_string(request.code.as_ref());
        match self
            .session_repo
            .verify_and_consume_auth_code(user.id, &code_hash, MAX_AUTH_CODE_ATTEMPTS)
            .await?
        {
            AuthCodeVerification::Success => {}
            AuthCodeVerification::AttemptsExhausted => {
                return Err(SessionError::TooManyAttempts);
            }
            AuthCodeVerification::Invalid | AuthCodeVerification::NoActiveCode => {
                return Err(SessionError::Unauthorized);
            }
        }

        self.user_repo.verify_email(user.id).await?;

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

        let access_token = self.token_client.generate_gitdot_jwt(user.id, &user.name)?;
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

#[cfg(test)]
mod tests {
    use chrono::{Duration, Utc};

    use super::{GraceEntry, SessionService, SessionServiceImpl};
    use crate::{
        dto::{
            ExchangeGitHubCodeRequest, LogoutRequest, RefreshSessionRequest, SendAuthEmailRequest,
            VerifyAuthCodeRequest,
        },
        error::SessionError,
        repository::AuthCodeVerification,
        service::{
            test_client::{
                MockEmailClient, MockGitHubClient, MockImageClient, MockR2Client, MockRedisClient,
                MockTokenClient,
            },
            test_common::{create_github_email, create_session, create_user},
            test_repository::{MockSessionRepository, MockUserRepository},
        },
        util::crypto::hash_string,
    };

    type Service = SessionServiceImpl<
        MockSessionRepository,
        MockUserRepository,
        MockEmailClient,
        MockGitHubClient,
        MockTokenClient,
        MockImageClient,
        MockR2Client,
        MockRedisClient,
    >;

    fn create_service(
        session_repo: MockSessionRepository,
        redis_client: MockRedisClient,
        token_client: MockTokenClient,
    ) -> Service {
        SessionServiceImpl {
            session_repo,
            user_repo: MockUserRepository::new(),
            email_client: MockEmailClient::new(),
            github_client: MockGitHubClient::new(),
            token_client,
            image_client: MockImageClient::new(),
            r2_client: MockR2Client::new(),
            redis_client,
        }
    }

    fn create_default_service() -> Service {
        create_service(
            MockSessionRepository::default(),
            MockRedisClient::default(),
            MockTokenClient::default(),
        )
    }

    // Shared by both the `refresh_session` and `logout` tests.
    const REFRESH_TOKEN: &str = "raw-refresh-token";

    mod send_auth_email {
        use super::*;

        #[tokio::test]
        async fn send_auth_email_within_cooldown_is_too_many_attempts() {
            let email = "user@example.com";
            let rate_limit_key = format!("auth_code_send:{}", hash_string(email));
            // Pre-claimed key → the SET NX returns false → cooldown.
            let redis = MockRedisClient::default().seed(&rate_limit_key, &true);
            let service = create_service(
                MockSessionRepository::default(),
                redis,
                MockTokenClient::default(),
            );

            let err = service
                .send_auth_email(SendAuthEmailRequest::new(email).unwrap())
                .await
                .unwrap_err();

            assert!(matches!(err, SessionError::TooManyAttempts));
        }

        #[tokio::test]
        async fn send_auth_email_creates_user_when_none_exists() {
            let mut service = create_default_service();
            service
                .user_repo
                .expect_get_by_primary_email()
                .returning(|_| Ok(None));
            service
                .user_repo
                .expect_is_email_taken()
                .returning(|_| Ok(false));
            service
                .user_repo
                .expect_create()
                .returning(|_, _, _| Ok(create_user("newbie")));
            service
                .image_client
                .expect_generate_user_image()
                .returning(|_| Ok(bytes::Bytes::new()));
            service
                .r2_client
                .expect_upload_object()
                .returning(|_, _| Ok(()));
            service
                .email_client
                .expect_send_email()
                .returning(|_, _, _, _| Ok(()));

            service
                .send_auth_email(SendAuthEmailRequest::new("user@example.com").unwrap())
                .await
                .unwrap();

            assert_eq!(service.session_repo.created_auth_codes(), 1);
        }

        #[tokio::test]
        async fn send_auth_email_for_secondary_email_silently_succeeds() {
            let mut service = create_default_service();
            service
                .user_repo
                .expect_get_by_primary_email()
                .returning(|_| Ok(None));
            // Email exists as someone else's secondary → no user created, no code.
            service
                .user_repo
                .expect_is_email_taken()
                .returning(|_| Ok(true));

            service
                .send_auth_email(SendAuthEmailRequest::new("user@example.com").unwrap())
                .await
                .unwrap();

            assert_eq!(service.session_repo.created_auth_codes(), 0);
        }

        #[tokio::test]
        async fn send_auth_email_for_existing_user_reissues_code() {
            let user = create_user("alice");
            let uid = user.id;
            let mut service = create_default_service();
            service
                .user_repo
                .expect_get_by_primary_email()
                .returning(move |_| Ok(Some(user.clone())));
            service
                .email_client
                .expect_send_email()
                .returning(|_, _, _, _| Ok(()));

            service
                .send_auth_email(SendAuthEmailRequest::new("alice@example.com").unwrap())
                .await
                .unwrap();

            assert_eq!(service.session_repo.created_auth_codes(), 1);
            assert_eq!(service.session_repo.invalidated_users(), vec![uid]);
        }

        #[tokio::test]
        async fn send_auth_email_proceeds_when_rate_limit_check_errors() {
            let user = create_user("alice");
            // A failing Redis must not block the send (fail-open on rate limiting).
            let mut service = create_service(
                MockSessionRepository::default(),
                MockRedisClient::failing(),
                MockTokenClient::default(),
            );
            service
                .user_repo
                .expect_get_by_primary_email()
                .returning(move |_| Ok(Some(user.clone())));
            service
                .email_client
                .expect_send_email()
                .returning(|_, _, _, _| Ok(()));

            service
                .send_auth_email(SendAuthEmailRequest::new("alice@example.com").unwrap())
                .await
                .unwrap();

            assert_eq!(service.session_repo.created_auth_codes(), 1);
        }
    }

    mod verify_auth_code {
        use super::*;

        fn verify_request() -> VerifyAuthCodeRequest {
            VerifyAuthCodeRequest::new("alice@example.com", "ABC234", None, None).unwrap()
        }

        #[tokio::test]
        async fn verify_auth_code_unknown_user_is_unauthorized() {
            let mut service = create_default_service();
            service
                .user_repo
                .expect_get_by_primary_email()
                .returning(|_| Ok(None));

            let err = service
                .verify_auth_code(verify_request())
                .await
                .unwrap_err();
            assert!(matches!(err, SessionError::Unauthorized));
        }

        #[tokio::test]
        async fn verify_auth_code_exhausted_attempts_is_too_many_attempts() {
            let user = create_user("alice");
            let mut service = create_service(
                MockSessionRepository::default()
                    .with_verification(AuthCodeVerification::AttemptsExhausted),
                MockRedisClient::default(),
                MockTokenClient::default(),
            );
            service
                .user_repo
                .expect_get_by_primary_email()
                .returning(move |_| Ok(Some(user.clone())));

            let err = service
                .verify_auth_code(verify_request())
                .await
                .unwrap_err();
            assert!(matches!(err, SessionError::TooManyAttempts));
        }

        #[tokio::test]
        async fn verify_auth_code_invalid_code_is_unauthorized() {
            let user = create_user("alice");
            let mut service = create_service(
                MockSessionRepository::default().with_verification(AuthCodeVerification::Invalid),
                MockRedisClient::default(),
                MockTokenClient::default(),
            );
            service
                .user_repo
                .expect_get_by_primary_email()
                .returning(move |_| Ok(Some(user.clone())));

            let err = service
                .verify_auth_code(verify_request())
                .await
                .unwrap_err();
            assert!(matches!(err, SessionError::Unauthorized));
        }

        #[tokio::test]
        async fn verify_auth_code_no_active_code_is_unauthorized() {
            let user = create_user("alice");
            let mut service = create_service(
                MockSessionRepository::default()
                    .with_verification(AuthCodeVerification::NoActiveCode),
                MockRedisClient::default(),
                MockTokenClient::default(),
            );
            service
                .user_repo
                .expect_get_by_primary_email()
                .returning(move |_| Ok(Some(user.clone())));

            let err = service
                .verify_auth_code(verify_request())
                .await
                .unwrap_err();
            assert!(matches!(err, SessionError::Unauthorized));
        }

        #[tokio::test]
        async fn verify_auth_code_success_issues_tokens() {
            let user = create_user("alice");
            let mut service = create_service(
                MockSessionRepository::default().with_verification(AuthCodeVerification::Success),
                MockRedisClient::default(),
                MockTokenClient::default(),
            );
            service
                .user_repo
                .expect_get_by_primary_email()
                .returning(move |_| Ok(Some(user.clone())));
            service
                .user_repo
                .expect_verify_email()
                .returning(|_| Ok(()));

            let resp = service.verify_auth_code(verify_request()).await.unwrap();

            assert_eq!(resp.access_token, "gitdot-jwt");
            assert_eq!(resp.refresh_token, "refresh-token-raw");
            // Previously-unverified email → flagged as a new sign-in.
            assert!(resp.is_new);
            assert_eq!(service.session_repo.created_sessions(), 1);
        }
    }

    mod refresh_session {
        use super::*;

        fn refresh_request() -> RefreshSessionRequest {
            RefreshSessionRequest::new(REFRESH_TOKEN.to_string(), None, None)
        }

        fn grace_key() -> String {
            format!("refresh_grace:{}", hash_string(REFRESH_TOKEN))
        }

        #[tokio::test]
        async fn refresh_session_unknown_token_is_not_found() {
            // session_repo default returns no session.
            let service = create_default_service();
            let err = service
                .refresh_session(refresh_request())
                .await
                .unwrap_err();
            assert!(matches!(err, SessionError::NotFound(_)));
        }

        #[tokio::test]
        async fn refresh_session_expired_token_is_token_expired() {
            let mut session = create_session(create_user("alice").id);
            session.expires_at = Utc::now() - Duration::hours(1);
            let service = create_service(
                MockSessionRepository::default().with_session(session),
                MockRedisClient::default(),
                MockTokenClient::default(),
            );

            let err = service
                .refresh_session(refresh_request())
                .await
                .unwrap_err();
            assert!(matches!(err, SessionError::TokenExpired(_)));
        }

        #[tokio::test]
        async fn refresh_session_replay_within_grace_returns_cached_tokens() {
            let user = create_user("alice");
            let mut session = create_session(user.id);
            session.revoked_at = Some(Utc::now()); // already rotated

            let grace = GraceEntry {
                refresh_token: "cached-refresh".to_string(),
                expires_at: Utc::now() + Duration::days(30),
            };
            let mut service = create_service(
                MockSessionRepository::default().with_session(session),
                MockRedisClient::default().seed(&grace_key(), &grace),
                MockTokenClient::default(),
            );
            service
                .user_repo
                .expect_get_by_id()
                .returning(move |_| Ok(Some(user.clone())));

            let resp = service.refresh_session(refresh_request()).await.unwrap();

            assert_eq!(resp.refresh_token, "cached-refresh");
            // Idempotent replay: no family revocation.
            assert!(service.session_repo.revoked_families().is_empty());
        }

        #[tokio::test]
        async fn refresh_session_replay_outside_grace_revokes_family() {
            let user = create_user("alice");
            let mut session = create_session(user.id);
            session.revoked_at = Some(Utc::now());
            let family = session.refresh_token_family;

            // Revoked token, no grace entry in Redis → real reuse.
            let mut service = create_service(
                MockSessionRepository::default().with_session(session),
                MockRedisClient::default(),
                MockTokenClient::default(),
            );
            service
                .user_repo
                .expect_get_by_id()
                .returning(move |_| Ok(Some(user.clone())));

            let err = service
                .refresh_session(refresh_request())
                .await
                .unwrap_err();

            assert!(matches!(err, SessionError::TokenRevoked(_)));
            assert_eq!(service.session_repo.revoked_families(), vec![family]);
        }

        #[tokio::test]
        async fn refresh_session_happy_path_rotates_tokens() {
            let user = create_user("alice");
            let session = create_session(user.id);
            let session_id = session.id;

            let mut service = create_service(
                MockSessionRepository::default().with_session(session),
                MockRedisClient::default(),
                MockTokenClient::default(),
            );
            service
                .user_repo
                .expect_get_by_id()
                .returning(move |_| Ok(Some(user.clone())));

            let resp = service.refresh_session(refresh_request()).await.unwrap();

            assert_eq!(resp.refresh_token, "refresh-token-raw");
            assert_eq!(service.session_repo.revoked_sessions(), vec![session_id]);
            assert_eq!(service.session_repo.created_sessions(), 1);
            // Replacement was cached under the grace key for replay tolerance.
            assert!(service.redis_client.contains(&grace_key()));
        }

        #[tokio::test]
        async fn refresh_session_lost_claim_replays_winners_tokens() {
            let user = create_user("alice");
            let session = create_session(user.id);

            // Another worker already claimed the rotation (grace key present).
            let winner = GraceEntry {
                refresh_token: "winner-refresh".to_string(),
                expires_at: Utc::now() + Duration::days(30),
            };
            let mut service = create_service(
                MockSessionRepository::default().with_session(session),
                MockRedisClient::default().seed(&grace_key(), &winner),
                MockTokenClient::default(),
            );
            service
                .user_repo
                .expect_get_by_id()
                .returning(move |_| Ok(Some(user.clone())));

            let resp = service.refresh_session(refresh_request()).await.unwrap();

            assert_eq!(resp.refresh_token, "winner-refresh");
            // Lost the claim → did no writes itself.
            assert_eq!(service.session_repo.created_sessions(), 0);
            assert!(service.session_repo.revoked_sessions().is_empty());
        }

        #[tokio::test]
        async fn refresh_session_lost_claim_with_vanished_cache_revokes_family() {
            let user = create_user("alice");
            let session = create_session(user.id);
            let family = session.refresh_token_family;

            let mut service = create_service(
                MockSessionRepository::default().with_session(session),
                MockRedisClient::phantom_nx_conflict(),
                MockTokenClient::default(),
            );
            service
                .user_repo
                .expect_get_by_id()
                .returning(move |_| Ok(Some(user.clone())));

            let err = service
                .refresh_session(refresh_request())
                .await
                .unwrap_err();

            assert!(matches!(err, SessionError::TokenRevoked(_)));
            assert_eq!(service.session_repo.revoked_families(), vec![family]);
        }
    }

    mod logout {
        use super::*;

        #[tokio::test]
        async fn logout_unknown_token_is_not_found() {
            let service = create_default_service();
            let err = service
                .logout(LogoutRequest {
                    refresh_token: REFRESH_TOKEN.to_string(),
                })
                .await
                .unwrap_err();
            assert!(matches!(err, SessionError::NotFound(_)));
        }

        #[tokio::test]
        async fn logout_revokes_single_session() {
            let session = create_session(create_user("alice").id);
            let session_id = session.id;
            let service = create_service(
                MockSessionRepository::default().with_session(session),
                MockRedisClient::default(),
                MockTokenClient::default(),
            );

            service
                .logout(LogoutRequest {
                    refresh_token: REFRESH_TOKEN.to_string(),
                })
                .await
                .unwrap();

            assert_eq!(service.session_repo.revoked_sessions(), vec![session_id]);
            // Only the one session, never the whole family.
            assert!(service.session_repo.revoked_families().is_empty());
        }
    }

    mod get_github_authorization_url {
        use super::*;

        #[tokio::test]
        async fn get_github_authorization_url_embeds_state() {
            let mut service = create_default_service();
            service
                .github_client
                .expect_get_authorization_url()
                .returning(|state| {
                    format!("https://github.com/login/oauth/authorize?state={state}")
                });

            let resp = service.get_github_authorization_url();

            assert_eq!(resp.state, "oauth-state");
            assert!(resp.authorize_url.contains("oauth-state"));
        }
    }

    mod exchange_github_code {
        use super::*;

        fn exchange_request() -> ExchangeGitHubCodeRequest {
            ExchangeGitHubCodeRequest::new(
                "gh-code".to_string(),
                "gh-state".to_string(),
                None,
                None,
            )
        }

        #[tokio::test]
        async fn exchange_github_code_invalid_state_is_unauthorized() {
            let service = create_service(
                MockSessionRepository::default(),
                MockRedisClient::default(),
                MockTokenClient::default().with_invalid_oauth_state(),
            );

            let err = service
                .exchange_github_code(exchange_request())
                .await
                .unwrap_err();
            assert!(matches!(err, SessionError::Unauthorized));
        }

        #[tokio::test]
        async fn exchange_github_code_without_verified_primary_is_github_error() {
            let mut service = create_default_service();
            service
                .github_client
                .expect_exchange_code()
                .returning(|_| Ok("gh-token".to_string()));
            service
                .github_client
                .expect_get_user_emails()
                // Verified but not primary → no usable primary email.
                .returning(|_| Ok(vec![create_github_email("alice@example.com", false, true)]));

            let err = service
                .exchange_github_code(exchange_request())
                .await
                .unwrap_err();
            assert!(matches!(err, SessionError::GitHubError(_)));
        }

        #[tokio::test]
        async fn exchange_github_code_secondary_email_is_unauthorized() {
            let mut service = create_default_service();
            service
                .github_client
                .expect_exchange_code()
                .returning(|_| Ok("gh-token".to_string()));
            service
                .github_client
                .expect_get_user_emails()
                .returning(|_| Ok(vec![create_github_email("alice@example.com", true, true)]));
            service
                .user_repo
                .expect_get_by_primary_email()
                .returning(|_| Ok(None));
            // Already verified as another account's secondary.
            service
                .user_repo
                .expect_is_email_taken()
                .returning(|_| Ok(true));

            let err = service
                .exchange_github_code(exchange_request())
                .await
                .unwrap_err();
            assert!(matches!(err, SessionError::Unauthorized));
        }

        #[tokio::test]
        async fn exchange_github_code_new_user_upserts_extra_emails() {
            let mut service = create_default_service();
            service
                .github_client
                .expect_exchange_code()
                .returning(|_| Ok("gh-token".to_string()));
            service
                .github_client
                .expect_get_user_emails()
                .returning(|_| {
                    Ok(vec![
                        create_github_email("alice@example.com", true, true),
                        create_github_email("alice.work@example.com", false, true),
                    ])
                });
            service
                .user_repo
                .expect_get_by_primary_email()
                .returning(|_| Ok(None));
            service
                .user_repo
                .expect_is_email_taken()
                .returning(|_| Ok(false));
            service
                .user_repo
                .expect_create()
                .returning(|_, _, _| Ok(create_user("alice")));
            service
                .user_repo
                .expect_upsert_verified_emails()
                .returning(|_, _| Ok(()));

            let resp = service
                .exchange_github_code(exchange_request())
                .await
                .unwrap();

            assert!(resp.is_new);
            assert_eq!(resp.access_token, "gitdot-jwt");
            assert_eq!(resp.refresh_token, "refresh-token-raw");
            assert_eq!(service.session_repo.created_sessions(), 1);
        }

        #[tokio::test]
        async fn exchange_github_code_existing_user_signs_in() {
            let user = create_user("alice");
            let mut service = create_default_service();
            service
                .github_client
                .expect_exchange_code()
                .returning(|_| Ok("gh-token".to_string()));
            service
                .github_client
                .expect_get_user_emails()
                .returning(|_| Ok(vec![create_github_email("alice@example.com", true, true)]));
            service
                .user_repo
                .expect_get_by_primary_email()
                .returning(move |_| Ok(Some(user.clone())));

            let resp = service
                .exchange_github_code(exchange_request())
                .await
                .unwrap();

            assert!(!resp.is_new);
            assert_eq!(service.session_repo.created_sessions(), 1);
        }
    }
}
