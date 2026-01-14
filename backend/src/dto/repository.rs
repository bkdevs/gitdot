use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

fn default_branch() -> String {
    "main".to_string()
}

fn default_ref() -> String {
    "HEAD".to_string()
}

fn default_page() -> u32 {
    1
}

fn default_per_page() -> u32 {
    30
}

#[derive(Deserialize)]
pub struct CreateRepositoryRequest {
    #[serde(default = "default_branch")]
    pub default_branch: String,
}

#[derive(Serialize)]
pub struct CreateRepositoryResponse {
    pub owner: String,
    pub name: String,
    pub default_branch: String,
}

#[derive(Deserialize)]
pub struct RepositoryTreeQuery {
    #[serde(default = "default_ref")]
    pub ref_name: String,
    #[serde(default)]
    pub path: String,
}

#[derive(Serialize)]
pub struct RepositoryTree {
    pub ref_name: String,
    pub commit_sha: String,
    pub path: String,
    pub entries: Vec<RepositoryTreeEntry>,
}

#[derive(Serialize)]
pub struct RepositoryTreeEntry {
    pub path: String,
    pub name: String,
    pub entry_type: String,
    pub sha: String,
    pub commit: RepositoryCommit,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<String>,
}

#[derive(Deserialize)]
pub struct RepositoryFileQuery {
    #[serde(default = "default_ref")]
    pub ref_name: String,
    pub path: String,
}

#[derive(Serialize, Clone)]
pub struct RepositoryFile {
    pub ref_name: String,
    pub commit_sha: String,
    pub path: String,
    pub sha: String,
    pub content: String,
    pub encoding: String,
}

#[derive(Deserialize)]
pub struct RepositoryCommitsQuery {
    #[serde(default = "default_ref")]
    pub ref_name: String,
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_per_page")]
    pub per_page: u32,
}

#[derive(Deserialize)]
pub struct RepositoryFileCommitsQuery {
    pub path: String,
    #[serde(default = "default_ref")]
    pub ref_name: String,
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_per_page")]
    pub per_page: u32,
}

#[derive(Serialize)]
pub struct RepositoryCommits {
    pub commits: Vec<RepositoryCommit>,
    pub has_next: bool,
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
    pub old_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left: Option<RepositoryFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right: Option<RepositoryFile>,

    pub lines_added: u32,
    pub lines_removed: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub diff: Option<FileDiff>,
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

pub type DiffChunk = Vec<DiffLine>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDiff {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunks: Option<Vec<DiffChunk>>,
    pub language: String,
    pub status: DiffStatus,
}
