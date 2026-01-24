use thiserror::Error;

#[derive(Debug, Error)]
pub enum OrganizationError {
    #[error("Organization with name {0} already exists")]
    Duplicate(String),

    #[error("Organization with name {0} does not exist")]
    NotFound(String),

    #[error("Invalid organization name: {0}")]
    InvalidOrganizationName(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}
