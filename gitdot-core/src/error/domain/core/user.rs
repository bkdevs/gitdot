use thiserror::Error;

use crate::error::{ConflictError, DatabaseError, InputError, NotFoundError};

#[derive(Debug, Error)]
pub enum UserError {
    #[error(transparent)]
    Input(#[from] InputError),

    #[error(transparent)]
    NotFound(#[from] NotFoundError),

    #[error(transparent)]
    Conflict(#[from] ConflictError),

    #[error("Invalid image: {0}")]
    InvalidImage(String),

    #[error(transparent)]
    DatabaseError(#[from] DatabaseError),
}
