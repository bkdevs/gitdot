use thiserror::Error;

use crate::error::{InputError, NotFoundError};

#[derive(Debug, Error)]
pub enum AuthorizationError {
    #[error("Unauthorized")]
    Unauthorized,

    #[error(transparent)]
    Input(#[from] InputError),

    #[error(transparent)]
    NotFound(#[from] NotFoundError),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}
