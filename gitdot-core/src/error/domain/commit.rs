use thiserror::Error;

use crate::error::{DiffError, GitError, InputError, NotFoundError};

#[derive(Debug, Error)]
pub enum CommitError {
    #[error(transparent)]
    Input(#[from] InputError),

    #[error(transparent)]
    NotFound(#[from] NotFoundError),

    #[error("Git error: {0}")]
    GitError(#[from] GitError),

    #[error("Diff error: {0}")]
    DiffError(#[from] DiffError),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}
