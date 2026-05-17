use thiserror::Error;

use crate::error::{DatabaseError, GitError, InputError, NotFoundError};

#[derive(Debug, Error)]
pub enum CommitError {
    #[error(transparent)]
    Input(#[from] InputError),

    #[error(transparent)]
    NotFound(#[from] NotFoundError),

    #[error(transparent)]
    GitError(#[from] GitError),

    #[error(transparent)]
    DatabaseError(#[from] DatabaseError),
}
