use thiserror::Error;

use crate::error::{EmailError, GitHubError, InputError, NotFoundError};

#[derive(Debug, Error)]
pub enum AuthenticationError {
    #[error(transparent)]
    Input(#[from] InputError),

    #[error(transparent)]
    NotFound(#[from] NotFoundError),

    #[error(transparent)]
    Extraction(#[from] TokenExtractionError),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Token expired: {0}")]
    TokenExpired(String),

    #[error("Token revoked: {0}")]
    TokenRevoked(String),

    #[error("Token pending: {0}")]
    TokenPending(String),

    #[error(transparent)]
    EmailError(#[from] EmailError),

    #[error(transparent)]
    GitHubError(#[from] GitHubError),

    #[error(transparent)]
    TokenClientError(#[from] crate::error::TokenError),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

#[derive(Debug, Error)]
pub enum TokenExtractionError {
    #[error("Missing authorization header")]
    MissingHeader,

    #[error("Invalid authorization header format")]
    InvalidHeaderFormat,

    #[error("Invalid public key: {0}")]
    InvalidPublicKey(String),

    #[error("Invalid token: {0}")]
    InvalidToken(String),
}
