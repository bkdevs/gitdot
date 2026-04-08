use thiserror::Error;

use crate::error::{ConflictError, DatabaseError, ImageError, InputError, NotFoundError};

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
    DatabaseError(#[from] DatabaseError),
}
