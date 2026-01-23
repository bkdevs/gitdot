use thiserror::Error;

use super::GitError as Git2Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Repository '{0}' already exists for this owner")]
    Duplicate(String),

    #[error("Owner not found: {0}")]
    OwnerNotFound(String),

    #[error("Git error: {0}")]
    GitError(#[from] Git2Error),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}
