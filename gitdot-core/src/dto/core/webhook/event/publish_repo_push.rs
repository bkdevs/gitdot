use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct PublishRepoPushRequest {
    pub owner: String,
    pub repo: String,
    pub ref_name: String,
    pub old_sha: String,
    pub new_sha: String,
    pub pusher_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoPushEvent {
    pub owner: String,
    pub repo: String,
    pub ref_name: String,
    pub old_sha: String,
    pub new_sha: String,
    pub pusher_id: Uuid,
    pub pusher_name: String,
    pub commits: Vec<RepoPushCommit>,
    pub pushed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoPushCommit {
    pub sha: String,
    pub message: String,
}
