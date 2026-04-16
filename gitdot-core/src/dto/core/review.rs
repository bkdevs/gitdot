mod add_review_reviewer;
mod get_review;
mod get_review_diff;
mod judge_review_diff;
mod list_reviews;
mod merge_review_diff;
mod process_review;
mod publish_review;
mod remove_review_reviewer;
mod resolve_review_comment;
mod update_review;
mod update_review_comment;
mod update_review_diff;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::model::{
    CommentSide, Diff, DiffStatus, Review, ReviewComment, ReviewStatus, ReviewVerdict, Revision,
    User, Verdict,
};

use super::RepositoryDiffFileResponse;

pub use add_review_reviewer::AddReviewReviewerReqeuest;
pub use get_review::GetReviewRequest;
pub use get_review_diff::GetReviewDiffRequest;
pub use judge_review_diff::{DiffComment, JudgeAction, JudgeReviewDiffRequest};
pub use list_reviews::ListReviewsRequest;
pub use merge_review_diff::MergeReviewDiffRequest;
pub use process_review::ProcessReviewRequest;
pub use publish_review::PublishReviewRequest;
pub use remove_review_reviewer::RemoveReviewReviewerRequest;
pub use resolve_review_comment::ResolveReviewCommentRequest;
pub use update_review::UpdateReviewRequest;
pub use update_review_comment::UpdateReviewCommentRequest;
pub use update_review_diff::UpdateReviewDiffRequest;

#[derive(Debug, Clone)]
pub enum ReviewId {
    Number(i32),
    Hex(String),
}

#[derive(Debug, Clone)]
pub struct ReviewsResponse {
    pub reviews: Vec<ReviewResponse>,
}

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
    pub message: String,
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
            message: diff.message,
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
    pub parent_hash: String,
    pub created_at: DateTime<Utc>,
    pub verdicts: Vec<ReviewVerdictResponse>,
}

impl From<Revision> for RevisionResponse {
    fn from(revision: Revision) -> Self {
        Self {
            id: revision.id,
            diff_id: revision.diff_id,
            number: revision.number,
            commit_hash: revision.commit_hash,
            parent_hash: revision.parent_hash,
            created_at: revision.created_at,
            verdicts: revision
                .verdicts
                .unwrap_or_default()
                .into_iter()
                .map(ReviewVerdictResponse::from)
                .collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ReviewVerdictResponse {
    pub id: Uuid,
    pub diff_id: Uuid,
    pub revision_id: Uuid,
    pub reviewer_id: Uuid,
    pub verdict: String,
    pub created_at: DateTime<Utc>,
}

impl From<ReviewVerdict> for ReviewVerdictResponse {
    fn from(v: ReviewVerdict) -> Self {
        Self {
            id: v.id,
            diff_id: v.diff_id,
            revision_id: v.revision_id,
            reviewer_id: v.reviewer_id,
            verdict: verdict_to_string(v.verdict),
            created_at: v.created_at,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ReviewerResponse {
    pub id: Uuid,
    pub review_id: Uuid,
    pub reviewer_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub user: Option<ReviewAuthorResponse>,
}

impl From<crate::model::Reviewer> for ReviewerResponse {
    fn from(reviewer: crate::model::Reviewer) -> Self {
        Self {
            id: reviewer.id,
            review_id: reviewer.review_id,
            reviewer_id: reviewer.reviewer_id,
            created_at: reviewer.created_at,
            user: reviewer.user.map(ReviewAuthorResponse::from),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ReviewCommentResponse {
    pub id: Uuid,
    pub review_id: Uuid,
    pub diff_id: Uuid,
    pub revision_id: Uuid,
    pub author_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub body: String,
    pub file_path: Option<String>,
    pub line_number_start: Option<i32>,
    pub line_number_end: Option<i32>,
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
            line_number_start: comment.line_number_start,
            line_number_end: comment.line_number_end,
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
        ReviewStatus::InProgress => "in_progress".to_string(),
        ReviewStatus::Closed => "closed".to_string(),
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

fn side_to_string(side: CommentSide) -> String {
    match side {
        CommentSide::Old => "old".to_string(),
        CommentSide::New => "new".to_string(),
    }
}

fn verdict_to_string(verdict: Verdict) -> String {
    match verdict {
        Verdict::Approved => "approved".to_string(),
        Verdict::ChangesRequested => "changes_requested".to_string(),
    }
}

#[derive(Debug, Clone)]
pub struct ReviewDiffResponse {
    pub files: Vec<RepositoryDiffFileResponse>,
}
