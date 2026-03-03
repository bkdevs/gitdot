use async_trait::async_trait;
use chrono::Utc;
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};

use crate::{
    dto::{GITDOT_SERVER_ID, IssueTaskJwtRequest, IssueTaskJwtResponse, JwtClaims, S2_SERVER_ID, ValidateTokenRequest, ValidateTokenResponse},
    error::AuthorizationError,
    repository::{TokenRepository, TokenRepositoryImpl},
    util::token::{hash_token, validate_token_format},
};

#[async_trait]
pub trait AuthenticationService: Send + Sync + 'static {
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
pub struct AuthenticationServiceImpl<T>
where
    T: TokenRepository,
{
    token_repo: T,
    gitdot_private_key: String,
}

impl AuthenticationServiceImpl<TokenRepositoryImpl> {
    pub fn new(token_repo: TokenRepositoryImpl, gitdot_private_key: String) -> Self {
        Self {
            token_repo,
            gitdot_private_key,
        }
    }
}

#[async_trait]
impl<T> AuthenticationService for AuthenticationServiceImpl<T>
where
    T: TokenRepository,
{
    async fn validate_token(
        &self,
        request: ValidateTokenRequest,
    ) -> Result<ValidateTokenResponse, AuthorizationError> {
        if !validate_token_format(&request.token) {
            return Err(AuthorizationError::Unauthorized);
        }
        if !&request.token.starts_with(request.token_type.prefix()) {
            return Err(AuthorizationError::Unauthorized);
        }

        let token_hash = hash_token(&request.token);
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
            exp: now + 3600,
        };

        let encoding_key = EncodingKey::from_ed_pem(self.gitdot_private_key.as_bytes())
            .map_err(|e| AuthorizationError::InvalidPublicKey(e.to_string()))?;

        let token = encode(&Header::new(Algorithm::EdDSA), &claims, &encoding_key)
            .map_err(|e| AuthorizationError::InvalidToken(e.to_string()))?;

        Ok(IssueTaskJwtResponse { token })
    }
}
