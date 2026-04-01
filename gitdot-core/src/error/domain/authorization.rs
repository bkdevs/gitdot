use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthorizationError {
    #[error("Missing Authorization header")]
    MissingHeader,

    #[error("Invalid Authorization header format")]
    InvalidHeaderFormat,

    #[error("Invalid public key: {0}")]
    InvalidPublicKey(String),

    #[error("Invalid token: {0}")]
    InvalidToken(String),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}
