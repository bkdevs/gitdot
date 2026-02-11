use thiserror::Error;

#[derive(Debug, Error)]
pub enum DagError {
    #[error("Invalid owner name: {0}")]
    InvalidOwnerName(String),

    #[error("Invalid repository name: {0}")]
    InvalidRepositoryName(String),

    #[error("Dag not found: {0}")]
    NotFound(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}
