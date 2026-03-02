use async_trait::async_trait;

use crate::{
    dto::{ValidateTokenRequest, ValidateTokenResponse},
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
}

#[derive(Debug, Clone)]
pub struct AuthenticationServiceImpl<T>
where
    T: TokenRepository,
{
    token_repo: T,
}

impl AuthenticationServiceImpl<TokenRepositoryImpl> {
    pub fn new(token_repo: TokenRepositoryImpl) -> Self {
        Self { token_repo }
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
}
