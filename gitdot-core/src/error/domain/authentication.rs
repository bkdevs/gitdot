use thiserror::Error;

use crate::error::{EmailError, GitHubError, InputError};

#[derive(Debug, Error)]
pub enum AuthenticationError {
    #[error(transparent)]
    Input(#[from] InputError),

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

    #[error("Missing authorization header")]
    MissingAuthHeader,

    #[error("Invalid authorization header format")]
    InvalidAuthHeaderFormat,

    #[error("Invalid public key: {0}")]
    InvalidPublicKey(String),

    #[error("Invalid token: {0}")]
    InvalidToken(String),

    #[error("JWT error: {0}")]
    JwtError(String),

    #[error(transparent)]
    EmailError(#[from] EmailError),

    #[error(transparent)]
    GitHubError(#[from] GitHubError),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}
