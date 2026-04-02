use thiserror::Error;
use tokio::task::JoinError;

use crate::error::{DatabaseError, GitError, InputError, NotFoundError};

#[derive(Debug, Error)]
pub enum BuildError {
    #[error(transparent)]
    Input(#[from] InputError),

    #[error(transparent)]
    NotFound(#[from] NotFoundError),

    #[error("Invalid build config: {0}")]
    InvalidConfig(String),

    #[error(transparent)]
    GitError(GitError),

    #[error("Task join error: {0}")]
    JoinError(#[from] JoinError),

    #[error(transparent)]
    DatabaseError(#[from] DatabaseError),

    #[error("S2 error: {0}")]
    S2Error(String),
}
