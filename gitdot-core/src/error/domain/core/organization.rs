use thiserror::Error;

use crate::error::{ConflictError, InputError, NotFoundError};

#[derive(Debug, Error)]
pub enum OrganizationError {
    #[error(transparent)]
    Input(#[from] InputError),

    #[error(transparent)]
    NotFound(#[from] NotFoundError),

    #[error(transparent)]
    Conflict(#[from] ConflictError),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}
