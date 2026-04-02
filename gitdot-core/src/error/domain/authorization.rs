use thiserror::Error;

use crate::error::JwtError;

#[derive(Debug, Error)]
pub enum AuthorizationError {
    #[error(transparent)]
    Jwt(#[from] JwtError),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}
