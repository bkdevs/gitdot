use thiserror::Error;

use crate::error::{
    ConflictError, DatabaseError, GitError, GitHubError, InputError, NotFoundError,
};

#[derive(Debug, Error)]
pub enum MigrationError {
    #[error(transparent)]
    Input(#[from] InputError),

    #[error(transparent)]
    NotFound(#[from] NotFoundError),

    #[error(transparent)]
    Conflict(#[from] ConflictError),

    #[error(transparent)]
    GitError(#[from] GitError),

    #[error(transparent)]
    GitHubError(#[from] GitHubError),

    #[error(transparent)]
    DatabaseError(#[from] DatabaseError),
}
