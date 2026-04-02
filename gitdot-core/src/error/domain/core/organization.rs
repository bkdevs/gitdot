use thiserror::Error;

use crate::error::{ConflictError, DatabaseError, InputError, NotFoundError};

#[derive(Debug, Error)]
pub enum OrganizationError {
    #[error(transparent)]
    Input(#[from] InputError),

    #[error(transparent)]
    NotFound(#[from] NotFoundError),

    #[error(transparent)]
    Conflict(#[from] ConflictError),

    #[error(transparent)]
    DatabaseError(#[from] DatabaseError),
}
