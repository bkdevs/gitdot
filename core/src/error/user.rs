use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}
