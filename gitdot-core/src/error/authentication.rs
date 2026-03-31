use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthenticationError {
    #[error("Invalid email: {0}")]
    InvalidEmail(String),

    #[error("Email error: {0}")]
    EmailError(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}
