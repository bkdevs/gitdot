use thiserror::Error;

use super::{DiffError, GitError};

#[derive(Debug, Error)]
pub enum CommitError {
    #[error("Invalid owner name: {0}")]
    InvalidOwnerName(String),

    #[error("Invalid repository name: {0}")]
    InvalidRepositoryName(String),

    #[error("Invalid date range: {0}")]
    InvalidDateRange(String),

    #[error("Commit not found: {0}")]
    NotFound(String),

    #[error("Repository not found: {0}")]
    RepositoryNotFound(String),

    #[error("Git error: {0}")]
    GitError(#[from] GitError),

    #[error("Diff error: {0}")]
    DiffError(#[from] DiffError),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}
