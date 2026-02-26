use thiserror::Error;

#[derive(Debug, Error)]
pub enum TokenError {
    #[error("Authorization pending")]
    AuthorizationPending,

    #[error("Expired token")]
    ExpiredToken,

    #[error("Access denied")]
    AccessDenied,

    #[error("Invalid token type")]
    InvalidTokenType,

    #[error("Invalid device code")]
    InvalidDeviceCode,

    #[error("Invalid user code: {0}")]
    InvalidUserCode(String),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}
