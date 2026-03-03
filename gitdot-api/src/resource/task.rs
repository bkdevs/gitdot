use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskLogResource {
    pub seq_num: u64,
    pub timestamp: u64,
    pub body: String,
    pub stream: Option<String>,
    pub finished: Option<String>,
}

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskResource {
    pub id: Uuid,
    pub repository_id: Uuid,
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
    pub token: String,

    pub owner_name: String,
    pub repository_name: String,
    pub s2_uri: String,

    pub name: String,
    pub command: String,
    pub status: String,
}
