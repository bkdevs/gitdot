use thiserror::Error;

#[derive(Debug, Error)]
pub enum TaskError {
    #[error("Invalid owner name: {0}")]
    InvalidOwnerName(String),

    #[error("Invalid repository name: {0}")]
    InvalidRepositoryName(String),

    #[error("Invalid task status: {0}")]
    InvalidStatus(String),

    #[error("Task not found: {0}")]
    NotFound(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}
