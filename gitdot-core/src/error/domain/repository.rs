use thiserror::Error;

use crate::error::{ConflictError, DiffError, GitError as Git2Error, InputError, NotFoundError};

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error(transparent)]
    Input(#[from] InputError),

    #[error(transparent)]
    NotFound(#[from] NotFoundError),

    #[error(transparent)]
    Conflict(#[from] ConflictError),

    #[error("When 'refs' is specified, 'paths' must contain exactly one entry")]
    TooManyPaths,

    #[error("Path '{0}' is not a file")]
    NotAFile(String),

    #[error(transparent)]
    GitError(Git2Error),

    #[error(transparent)]
    DiffError(#[from] DiffError),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

impl From<Git2Error> for RepositoryError {
    fn from(e: Git2Error) -> Self {
        match e {
            Git2Error::NotFound(path) => {
                RepositoryError::NotFound(NotFoundError::new("repository", path))
            }
            other => RepositoryError::GitError(other),
        }
    }
}
