mod get_review;
mod list_reviews;
mod process_review;

use chrono::{DateTime, Utc};
use nutype::nutype;
use uuid::Uuid;

use crate::model::{
    CommentSide, Diff, DiffStatus, Review, ReviewComment, ReviewStatus, ReviewerStatus, Revision,
    User,
};

pub use get_review::GetReviewRequest;
pub use list_reviews::ListReviewsRequest;
pub use process_review::ProcessReviewRequest;

use crate::util::review::MAGIC_REF_PREFIX;

/// Validates that a ref is either `refs/for/<branch>` (create)
/// or `refs/for/<branch>/<review_number>` (update).
fn is_review_ref(s: &str) -> bool {
    let Some(rest) = s
        .strip_prefix(MAGIC_REF_PREFIX)
        .and_then(|r| r.strip_prefix('/'))
    else {
        return false;
    };

    if rest.is_empty() {
        return false;
    }

    // If the last segment is numeric, it's an update (refs/for/branch/123)
    // and we need at least one segment before it for the branch name.
    // Otherwise, the entire rest is a branch name (refs/for/branch).
    match rest.rsplit_once('/') {
        Some((branch, number)) => {
            if number.parse::<i64>().is_ok() {
                !branch.is_empty()
            } else {
                true
            }
        }
        None => true,
    }
}

#[nutype(
    validate(predicate = is_review_ref),
    derive(Debug, Clone, PartialEq, Eq, AsRef, Deref)
)]
pub struct ReviewRef(String);

#[derive(Debug, Clone)]
pub struct ReviewResponse {
    pub id: Uuid,
    pub number: i32,
    pub author_id: Uuid,
    pub repository_id: Uuid,
    pub title: String,
    pub description: String,
    pub target_branch: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub author: Option<ReviewAuthorResponse>,
    pub diffs: Vec<DiffResponse>,
    pub reviewers: Vec<ReviewerResponse>,
    pub comments: Vec<ReviewCommentResponse>,
}

impl From<Review> for ReviewResponse {
    fn from(review: Review) -> Self {
        Self {
            id: review.id,
            number: review.number,
            author_id: review.author_id,
            repository_id: review.repository_id,
            title: review.title,
            description: review.description,
            target_branch: review.target_branch,
            status: status_to_string(review.status),
            created_at: review.created_at,
            updated_at: review.updated_at,
            author: review.author.map(ReviewAuthorResponse::from),
            diffs: review
                .diffs
                .unwrap_or_default()
                .into_iter()
                .map(DiffResponse::from)
                .collect(),
            reviewers: review
                .reviewers
                .unwrap_or_default()
                .into_iter()
                .map(ReviewerResponse::from)
                .collect(),
            comments: review
                .comments
                .unwrap_or_default()
                .into_iter()
                .map(ReviewCommentResponse::from)
                .collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ReviewAuthorResponse {
    pub id: Uuid,
    pub name: String,
}

impl From<User> for ReviewAuthorResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DiffResponse {
    pub id: Uuid,
    pub review_id: Uuid,
    pub position: i32,
    pub title: String,
    pub description: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub revisions: Vec<RevisionResponse>,
}

impl From<Diff> for DiffResponse {
    fn from(diff: Diff) -> Self {
        Self {
            id: diff.id,
            review_id: diff.review_id,
            position: diff.position,
            title: diff.title,
            description: diff.description,
            status: diff_status_to_string(diff.status),
            created_at: diff.created_at,
            updated_at: diff.updated_at,
            revisions: diff
                .revisions
                .unwrap_or_default()
                .into_iter()
                .map(RevisionResponse::from)
                .collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RevisionResponse {
    pub id: Uuid,
    pub diff_id: Uuid,
    pub number: i32,
    pub commit_hash: String,
    pub created_at: DateTime<Utc>,
}

impl From<Revision> for RevisionResponse {
    fn from(revision: Revision) -> Self {
        Self {
            id: revision.id,
            diff_id: revision.diff_id,
            number: revision.number,
            commit_hash: revision.commit_hash,
            created_at: revision.created_at,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ReviewerResponse {
    pub id: Uuid,
    pub review_id: Uuid,
    pub reviewer_id: Uuid,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub user: Option<ReviewAuthorResponse>,
}

impl From<crate::model::Reviewer> for ReviewerResponse {
    fn from(reviewer: crate::model::Reviewer) -> Self {
        Self {
            id: reviewer.id,
            review_id: reviewer.review_id,
            reviewer_id: reviewer.reviewer_id,
            status: reviewer_status_to_string(reviewer.status),
            created_at: reviewer.created_at,
            user: reviewer.user.map(ReviewAuthorResponse::from),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ReviewCommentResponse {
    pub id: Uuid,
    pub review_id: Uuid,
    pub diff_id: Option<Uuid>,
    pub revision_id: Option<Uuid>,
    pub author_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub body: String,
    pub file_path: Option<String>,
    pub line_number: Option<i32>,
    pub side: Option<String>,
    pub resolved: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub author: Option<ReviewAuthorResponse>,
}

impl From<ReviewComment> for ReviewCommentResponse {
    fn from(comment: ReviewComment) -> Self {
        Self {
            id: comment.id,
            review_id: comment.review_id,
            diff_id: comment.diff_id,
            revision_id: comment.revision_id,
            author_id: comment.author_id,
            parent_id: comment.parent_id,
            body: comment.body,
            file_path: comment.file_path,
            line_number: comment.line_number,
            side: comment.side.map(side_to_string),
            resolved: comment.resolved,
            created_at: comment.created_at,
            updated_at: comment.updated_at,
            author: comment.author.map(ReviewAuthorResponse::from),
        }
    }
}

fn status_to_string(status: ReviewStatus) -> String {
    match status {
        ReviewStatus::Draft => "draft".to_string(),
        ReviewStatus::Open => "open".to_string(),
        ReviewStatus::ChangesRequested => "changes_requested".to_string(),
        ReviewStatus::Approved => "approved".to_string(),
        ReviewStatus::Merged => "merged".to_string(),
    }
}

fn diff_status_to_string(status: DiffStatus) -> String {
    match status {
        DiffStatus::Open => "open".to_string(),
        DiffStatus::ChangesRequested => "changes_requested".to_string(),
        DiffStatus::Approved => "approved".to_string(),
        DiffStatus::Merged => "merged".to_string(),
    }
}

fn reviewer_status_to_string(status: ReviewerStatus) -> String {
    match status {
        ReviewerStatus::Pending => "pending".to_string(),
        ReviewerStatus::ChangesRequested => "changes_requested".to_string(),
        ReviewerStatus::Approved => "approved".to_string(),
    }
}

fn side_to_string(side: CommentSide) -> String {
    match side {
        CommentSide::Old => "old".to_string(),
        CommentSide::New => "new".to_string(),
    }
}
