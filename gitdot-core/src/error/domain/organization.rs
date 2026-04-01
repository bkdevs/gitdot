use thiserror::Error;

#[derive(Debug, Error)]
pub enum OrganizationError {
    #[error("Organization with name {0} already exists")]
    Duplicate(String),

    #[error("Organization with name {0} does not exist")]
    NotFound(String),

    #[error("Invalid organization name: {0}")]
    InvalidOrganizationName(String),

    #[error("User {0} is already a member of this organization")]
    MemberAlreadyExists(String),

    #[error("User {0} not found")]
    UserNotFound(String),

    #[error("Invalid user name: {0}")]
    InvalidUserName(String),

    #[error("Invalid role: {0}")]
    InvalidRole(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}
