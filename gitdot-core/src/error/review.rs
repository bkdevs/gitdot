use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReviewError {
    #[error("Invalid owner name: {0}")]
    InvalidOwnerName(String),

    #[error("Invalid repository name: {0}")]
    InvalidRepositoryName(String),

    #[error("Invalid ref name: {0}")]
    InvalidRefName(String),

    #[error("Review not found: {0}")]
    ReviewNotFound(String),

    #[error("Repository not found: {0}")]
    RepositoryNotFound(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}
