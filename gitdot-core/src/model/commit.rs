use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct Commit {
    pub id: Uuid,
    pub repo_id: Uuid,

    pub author_id: Option<Uuid>,
    pub git_author_name: String,
    pub git_author_email: String,

    pub ref_name: String,
    pub sha: String,
    pub message: String,
    pub created_at: DateTime<Utc>,

    #[sqlx(json)]
    pub diffs: Vec<CommitDiff>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitDiff {
    pub path: String,
    pub lines_added: i32,
    pub lines_removed: i32,
    pub hunks: Vec<CommitDiffHunk>,
}

pub type CommitDiffHunk = Vec<CommitDiffPair>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitDiffPair {
    pub lhs: Option<CommitDiffLine>,
    pub rhs: Option<CommitDiffLine>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitDiffLine {
    pub line_number: u32,
    pub changes: Vec<CommitDiffChange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitDiffChange {
    pub start: u32,
    pub end: u32,
    pub content: String,
    pub highlight: CommitDiffSyntaxHighlight,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommitDiffSyntaxHighlight {
    Delimiter,
    Normal,
    String,
    Type,
    Comment,
    Keyword,
    TreeSitterError,
}
