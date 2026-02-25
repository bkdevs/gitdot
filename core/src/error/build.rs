use thiserror::Error;

use crate::error::GitError;

#[derive(Debug, Error)]
pub enum BuildError {
    #[error("Invalid owner name: {0}")]
    InvalidOwnerName(String),

    #[error("Invalid repository name: {0}")]
    InvalidRepositoryName(String),

    #[error("Invalid trigger: {0}")]
    InvalidTrigger(String),

    #[error("Build not found: {0}")]
    NotFound(String),

    #[error(".gitdot-ci.toml not found at ref '{0}'")]
    ConfigNotFound(String),

    #[error("Invalid build config: {0}")]
    InvalidConfig(String),

    #[error("Git error: {0}")]
    GitError(GitError),

    #[error("Task join error: {0}")]
    JoinError(#[from] tokio::task::JoinError),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}
