use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthenticationError {
    #[error("Invalid email: {0}")]
    InvalidEmail(String),

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

    #[error("JWT error: {0}")]
    JwtError(String),

    #[error("Email error: {0}")]
    EmailError(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}
