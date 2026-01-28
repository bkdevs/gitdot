use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("Invalid user name: {0}")]
    InvalidUserName(String),

    #[error("User not found: {0}")]
    NotFound(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}
