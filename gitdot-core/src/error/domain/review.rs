use thiserror::Error;

use crate::error::{DiffError, GitError};

#[derive(Debug, Error)]
pub enum ReviewError {
    #[error("Invalid owner name: {0}")]
    InvalidOwnerName(String),

    #[error("Invalid repository name: {0}")]
    InvalidRepositoryName(String),

    #[error("Invalid ref name: {0}")]
    InvalidRefName(String),

    #[error("Review not found: {0}")]
    ReviewNotFound(String),

    #[error("Repository not found: {0}")]
    RepositoryNotFound(String),

    #[error("User not found: {0}")]
    UserNotFound(String),

    #[error("Cannot add review author as reviewer: {0}")]
    CannotReviewOwnReview(String),

    #[error("Reviewer already exists: {0}")]
    ReviewerAlreadyExists(String),

    #[error("Review is not publishable: {0}")]
    ReviewNotPublishable(String),

    #[error("Reviewer not found: {0}")]
    ReviewerNotFound(String),

    #[error("Diff not found: {0}")]
    DiffNotFound(String),

    #[error("Revision not found: {0}")]
    RevisionNotFound(String),

    #[error("Comment not found: {0}")]
    CommentNotFound(String),

    #[error("Invalid comment: {0}")]
    InvalidComment(String),

    #[error("Diff is not mergeable: {0}")]
    DiffNotMergeable(String),

    #[error("User is not an org admin: {0}")]
    NotOrgAdmin(String),

    #[error("No commits found between target branch and pushed ref")]
    CommitsNotFound,

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Git error: {0}")]
    GitError(#[from] GitError),

    #[error("Diff rendering error: {0}")]
    DiffError(#[from] DiffError),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}
