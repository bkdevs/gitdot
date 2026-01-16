use thiserror::Error;

#[derive(Debug, Error)]
pub enum OrganizationError {
    #[error("Organization with name {0} already exists")]
    Duplicate(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}
