use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

use crate::model::User;

#[derive(Debug, Clone, FromRow)]
pub struct Review {
    pub id: Uuid,
    pub repository_id: Uuid,
    pub number: i32,
    pub author_id: Uuid,
    pub title: String,
    pub description: String,
    pub target_branch: String,
    pub status: ReviewStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    #[sqlx(json(nullable))]
    pub author: Option<User>,

    #[sqlx(json(nullable))]
    pub diffs: Option<Vec<Diff>>,

    #[sqlx(json(nullable))]
    pub reviewers: Option<Vec<Reviewer>>,

    #[sqlx(json(nullable))]
    pub comments: Option<Vec<ReviewComment>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Type, Serialize, Deserialize)]
#[sqlx(type_name = "review_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ReviewStatus {
    Draft,
    InProgress,
    Closed,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Diff {
    pub id: Uuid,
    pub review_id: Uuid,
    pub position: i32,
    pub title: String,
    pub description: String,
    pub status: DiffStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    #[sqlx(json(nullable))]
    pub revisions: Option<Vec<Revision>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Type, Serialize, Deserialize)]
#[sqlx(type_name = "diff_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum DiffStatus {
    Open,
    ChangesRequested,
    Approved,
    Merged,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Revision {
    pub id: Uuid,
    pub diff_id: Uuid,
    pub number: i32,
    pub commit_hash: String,
    pub parent_hash: String,
    pub created_at: DateTime<Utc>,

    #[sqlx(json(nullable))]
    pub verdicts: Option<Vec<ReviewVerdict>>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Reviewer {
    pub id: Uuid,
    pub review_id: Uuid,
    pub reviewer_id: Uuid,
    pub created_at: DateTime<Utc>,

    #[sqlx(json(nullable))]
    pub user: Option<User>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ReviewVerdict {
    pub id: Uuid,
    pub diff_id: Uuid,
    pub revision_id: Uuid,
    pub reviewer_id: Uuid,
    pub verdict: Verdict,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Type, Serialize, Deserialize)]
#[sqlx(type_name = "verdict", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Verdict {
    Approved,
    ChangesRequested,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ReviewComment {
    pub id: Uuid,
    pub review_id: Uuid,
    pub diff_id: Uuid,
    pub revision_id: Uuid,
    pub author_id: Uuid,

    // for threaded comments
    pub parent_id: Option<Uuid>,

    pub body: String,
    pub file_path: Option<String>,
    pub line_number_start: Option<i32>,
    pub line_number_end: Option<i32>,
    pub side: Option<CommentSide>,
    pub resolved: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    #[sqlx(json(nullable))]
    pub author: Option<User>,
}

#[derive(Debug, Clone, PartialEq, Eq, Type, Serialize, Deserialize)]
#[sqlx(type_name = "comment_side", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum CommentSide {
    Old,
    New,
}
