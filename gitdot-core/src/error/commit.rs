use thiserror::Error;

use super::GitError;

#[derive(Debug, Error)]
pub enum CommitError {
    #[error("Invalid owner name: {0}")]
    InvalidOwnerName(String),

    #[error("Invalid repository name: {0}")]
    InvalidRepositoryName(String),

    #[error("Repository not found: {0}")]
    RepositoryNotFound(String),

    #[error("Git error: {0}")]
    GitError(#[from] GitError),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}
