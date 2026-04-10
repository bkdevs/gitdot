use thiserror::Error;

use crate::error::{ConflictError, DatabaseError, ImageError, InputError, NotFoundError, R2Error};

#[derive(Debug, Error)]
pub enum UserError {
    #[error(transparent)]
    Input(#[from] InputError),

    #[error(transparent)]
    NotFound(#[from] NotFoundError),

    #[error(transparent)]
    Conflict(#[from] ConflictError),

    #[error(transparent)]
    InvalidImage(#[from] ImageError),

    #[error(transparent)]
    R2Error(#[from] R2Error),

    #[error(transparent)]
    DatabaseError(#[from] DatabaseError),
}
