use thiserror::Error;

#[derive(Debug, Error)]
pub enum RunnerError {
    #[error("Invalid runner name: {0}")]
    InvalidRunnerName(String),

    #[error("Invalid owner name: {0}")]
    InvalidOwnerName(String),

    #[error("Invalid owner type: {0}")]
    InvalidOwnerType(String),

    #[error("Runner not found: {0}")]
    NotFound(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}
