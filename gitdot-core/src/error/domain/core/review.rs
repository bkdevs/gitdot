use thiserror::Error;

use crate::error::{ConflictError, DatabaseError, DiffError, GitError, InputError, NotFoundError};

#[derive(Debug, Error)]
pub enum ReviewError {
    #[error(transparent)]
    Input(#[from] InputError),

    #[error(transparent)]
    NotFound(#[from] NotFoundError),

    #[error(transparent)]
    Conflict(#[from] ConflictError),

    #[error("Cannot add review author as reviewer: {0}")]
    CannotReviewOwnReview(String),

    #[error("Cannot remove review author as reviewer: {0}")]
    CannotRemoveReviewAuthor(String),

    #[error("Cannot approve your own diff")]
    CannotApproveOwnDiff,

    #[error("Review is not publishable: {0}")]
    ReviewNotPublishable(String),

    #[error("Diff is not mergeable: {0}")]
    DiffNotMergeable(String),

    #[error("User is not an org admin: {0}")]
    NotOrgAdmin(String),

    #[error("No commits found between target branch and pushed ref")]
    CommitsNotFound,

    #[error("Invalid review identifier: expected an integer review number or 8-char hex short ID")]
    InvalidIdentifier,

    #[error(transparent)]
    GitError(#[from] GitError),

    #[error(transparent)]
    DiffError(#[from] DiffError),

    #[error(transparent)]
    DatabaseError(#[from] DatabaseError),
}
