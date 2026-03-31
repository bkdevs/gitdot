use async_trait::async_trait;
use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};

use crate::{
    client::{EmailClient, ResendClient, TokenClient, TokenClientImpl},
    dto::{
        GITDOT_SERVER_ID, IssueTaskJwtRequest, IssueTaskJwtResponse, JwtClaims, S2_SERVER_ID,
        SendAuthEmailRequest, ValidateTokenRequest, ValidateTokenResponse,
    },
    error::{AuthenticationError, AuthorizationError},
    repository::{
        SessionRepository, SessionRepositoryImpl, TokenRepository, TokenRepositoryImpl,
        UserRepository, UserRepositoryImpl,
    },
    util::{
        auth::{NOREPLY_EMAIL, get_auth_email},
        crypto::hash_string,
    },
};

#[async_trait]
pub trait AuthenticationService: Send + Sync + 'static {
    async fn send_auth_email(
        &self,
        request: SendAuthEmailRequest,
    ) -> Result<(), AuthenticationError>;

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
    gitdot_private_key: String,
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
        gitdot_private_key: String,
    ) -> Self {
        Self {
            session_repo,
            token_repo,
            user_repo,
            email_client,
            token_client,
            gitdot_private_key,
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

        let (code, code_hash) = self.token_client.generate_auth_token();
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

        let encoding_key = EncodingKey::from_ed_pem(self.gitdot_private_key.as_bytes())
            .map_err(|e| AuthorizationError::InvalidPublicKey(e.to_string()))?;

        let token = encode(&Header::new(Algorithm::EdDSA), &claims, &encoding_key)
            .map_err(|e| AuthorizationError::InvalidToken(e.to_string()))?;

        Ok(IssueTaskJwtResponse { token })
    }
}
