use thiserror::Error;

use crate::error::{DatabaseError, InputError, NotFoundError};

#[derive(Debug, Error)]
pub enum QuestionError {
    #[error(transparent)]
    Input(#[from] InputError),

    #[error(transparent)]
    NotFound(#[from] NotFoundError),

    #[error(transparent)]
    DatabaseError(#[from] DatabaseError),
}
