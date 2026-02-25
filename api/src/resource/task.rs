use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskResource {
    pub id: Uuid,
    pub repo_owner: String,
    pub repo_name: String,
    pub build_id: Uuid,
    pub s2_uri: String,
    pub name: String,
    pub command: String,
    pub status: String,
    pub waits_for: Vec<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PollTaskResource {
    pub id: Uuid,
    pub repo_owner: String,
    pub repo_name: String,
    pub s2_uri: String,
    pub name: String,
    pub command: String,
    pub status: String,
}
