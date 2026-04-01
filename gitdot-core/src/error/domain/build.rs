use thiserror::Error;

use crate::error::{GitError, InputError, NotFoundError};

#[derive(Debug, Error)]
pub enum BuildError {
    #[error(transparent)]
    Input(#[from] InputError),

    #[error(transparent)]
    NotFound(#[from] NotFoundError),

    #[error("Invalid build config: {0}")]
    InvalidConfig(String),

    #[error("Git error: {0}")]
    GitError(GitError),

    #[error("Task join error: {0}")]
    JoinError(#[from] tokio::task::JoinError),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("S2 error: {0}")]
    S2Error(String),
}
