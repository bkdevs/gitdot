use thiserror::Error;

#[derive(Debug, Error)]
pub enum CommitError {
    #[error("Repository not found: {0}")]
    RepositoryNotFound(String),

    #[error("Git error: {0}")]
    GitError(#[from] crate::error::GitError),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}
