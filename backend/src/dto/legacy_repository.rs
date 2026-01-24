use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone)]
pub struct RepositoryFile {
    pub ref_name: String,
    pub commit_sha: String,
    pub path: String,
    pub sha: String,
    pub content: String,
    pub encoding: String,
}

#[derive(Serialize, Clone)]
pub struct RepositoryCommit {
    pub sha: String,
    pub message: String,
    pub author: String,
    pub date: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct RepositoryCommitDiffs {
    pub sha: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_sha: Option<String>,
    pub commit: RepositoryCommit,
    pub diffs: Vec<RepositoryFileDiff>,
}

#[derive(Serialize, Clone)]
pub struct RepositoryFileDiff {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left: Option<RepositoryFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right: Option<RepositoryFile>,

    pub lines_added: u32,
    pub lines_removed: u32,
    pub hunks: Vec<DiffHunk>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DiffStatus {
    Unchanged,
    Changed,
    Created,
    Deleted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SyntaxHighlight {
    Delimiter,
    Normal,
    String,
    Type,
    Comment,
    Keyword,
    TreeSitterError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffChange {
    pub start: u32,
    pub end: u32,
    pub content: String,
    pub highlight: SyntaxHighlight,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffSide {
    pub line_number: u32,
    pub changes: Vec<DiffChange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffLine {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lhs: Option<DiffSide>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rhs: Option<DiffSide>,
}

pub type DiffHunk = Vec<DiffLine>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DifftasticOutput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunks: Option<Vec<DiffHunk>>,
    pub language: String,
    pub path: String,
    pub status: DiffStatus,
}
