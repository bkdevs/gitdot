use thiserror::Error;

#[derive(Debug, Error)]
pub enum CommitError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}
