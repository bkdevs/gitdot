use thiserror::Error;

use crate::error::{ConflictError, GitError, GitHubError, InputError, NotFoundError};

#[derive(Debug, Error)]
pub enum MigrationError {
    #[error(transparent)]
    Input(#[from] InputError),

    #[error(transparent)]
    NotFound(#[from] NotFoundError),

    #[error(transparent)]
    Conflict(#[from] ConflictError),

    #[error("Git error: {0}")]
    GitError(#[from] GitError),

    #[error("GitHub error: {0}")]
    GitHubError(#[from] GitHubError),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}
