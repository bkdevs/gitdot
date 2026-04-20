use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewResource {
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
    pub author: Option<ReviewAuthorResource>,
    pub diffs: Vec<DiffResource>,
    pub reviewers: Vec<ReviewerResource>,
    pub comments: Vec<ReviewCommentResource>,
}

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewAuthorResource {
    pub id: Uuid,
    pub name: String,
}

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiffResource {
    pub id: Uuid,
    pub review_id: Uuid,
    pub position: i32,
    pub message: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub revisions: Vec<RevisionResource>,
}

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RevisionResource {
    pub id: Uuid,
    pub diff_id: Uuid,
    pub number: i32,
    pub commit_hash: String,
    pub parent_hash: String,
    pub created_at: DateTime<Utc>,
    pub verdicts: Vec<ReviewVerdictResource>,
}

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewVerdictResource {
    pub id: Uuid,
    pub diff_id: Uuid,
    pub revision_id: Uuid,
    pub reviewer_id: Uuid,
    pub verdict: String,
    pub created_at: DateTime<Utc>,
}

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewerResource {
    pub id: Uuid,
    pub review_id: Uuid,
    pub reviewer_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub user: Option<ReviewAuthorResource>,
}

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewCommentResource {
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
    pub start_character: Option<i32>,
    pub end_character: Option<i32>,
    pub side: Option<String>,
    pub resolved: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub author: Option<ReviewAuthorResource>,
}
