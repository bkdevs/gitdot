use thiserror::Error;

use super::{DiffError, GitError as Git2Error};

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Repository '{0}' already exists for this owner")]
    Duplicate(String),

    #[error("Repository not found: {0}")]
    NotFound(String),

    #[error("Owner not found: {0}")]
    OwnerNotFound(String),

    #[error("Invalid owner name: {0}")]
    InvalidOwnerName(String),

    #[error("Invalid repository name: {0}")]
    InvalidRepositoryName(String),

    #[error("Invalid owner type: {0}")]
    InvalidOwnerType(String),

    #[error("Invalid visibility: {0}")]
    InvalidVisibility(String),

    #[error("Git error: {0}")]
    GitError(#[from] Git2Error),

    #[error("Diff error: {0}")]
    DiffError(#[from] DiffError),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}
