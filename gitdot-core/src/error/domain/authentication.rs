use thiserror::Error;

use crate::error::{EmailError, GitHubError, InputError, JwtError};

#[derive(Debug, Error)]
pub enum AuthenticationError {
    #[error(transparent)]
    Input(#[from] InputError),

    #[error(transparent)]
    Jwt(#[from] JwtError),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Auth code not found")]
    AuthCodeNotFound,

    #[error("Auth code already used")]
    AuthCodeAlreadyUsed,

    #[error("Auth code expired")]
    AuthCodeExpired,

    #[error("Session not found")]
    SessionNotFound,

    #[error("Session expired")]
    SessionExpired,

    #[error("Session revoked")]
    SessionRevoked,

    #[error("Invalid OAuth state: {0}")]
    InvalidOAuthState(String),

    #[error(transparent)]
    EmailError(#[from] EmailError),

    #[error(transparent)]
    GitHubError(#[from] GitHubError),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}
