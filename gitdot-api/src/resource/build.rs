use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildResource {
    pub id: Uuid,
    pub number: i32,
    pub repository_id: Uuid,
    pub ref_name: String,
    pub trigger: String,
    pub commit_sha: String,
    pub status: String,
    pub total_tasks: i32,
    pub completed_tasks: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
