use thiserror::Error;

use crate::error::{
    DatabaseError, EmailError, GitHubError, InputError, NotFoundError, SlackBotError, TokenError,
};

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
    SlackBotError(#[from] SlackBotError),

    #[error(transparent)]
    TokenError(#[from] TokenError),

    #[error(transparent)]
    DatabaseError(#[from] DatabaseError),
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
