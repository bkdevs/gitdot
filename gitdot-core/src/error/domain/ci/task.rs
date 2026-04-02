use thiserror::Error;

use crate::error::{InputError, NotFoundError};

#[derive(Debug, Error)]
pub enum TaskError {
    #[error(transparent)]
    Input(#[from] InputError),

    #[error(transparent)]
    NotFound(#[from] NotFoundError),

    #[error("No matching build config for trigger")]
    NoBuildConfig,

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}
