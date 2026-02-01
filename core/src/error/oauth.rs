use thiserror::Error;

#[derive(Debug, Error)]
pub enum OAuthError {
    #[error("Authorization pending")]
    AuthorizationPending,

    #[error("Slow down")]
    SlowDown,

    #[error("Expired token")]
    ExpiredToken,

    #[error("Access denied")]
    AccessDenied,

    #[error("Invalid device code")]
    InvalidDeviceCode,

    #[error("Invalid user code")]
    InvalidUserCode,

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}
