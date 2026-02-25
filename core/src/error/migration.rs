use thiserror::Error;

use super::{GitError, GitHubError};

#[derive(Debug, Error)]
pub enum MigrationError {
    #[error("User not found: {0}")]
    UserNotFound(String),

    #[error("Owner not found: {0}")]
    OwnerNotFound(String),

    #[error("Invalid repository name: {0}")]
    InvalidRepositoryName(String),

    #[error("Repository already exists: {0}")]
    RepositoryAlreadyExists(String),

    #[error("Git error: {0}")]
    GitError(#[from] GitError),

    #[error("GitHub error: {0}")]
    GitHubError(#[from] GitHubError),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}
