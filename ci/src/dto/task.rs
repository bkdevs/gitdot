use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use gitdot_core::dto::TaskResponse;

#[derive(Debug, Clone, Deserialize)]
pub struct PollTaskQuery {
    pub rid: Uuid,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateTaskServerRequest {
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TaskServerResponse {
    pub id: Uuid,
    pub repo_owner: String,
    pub repo_name: String,
    pub script: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<TaskResponse> for TaskServerResponse {
    fn from(response: TaskResponse) -> Self {
        Self {
            id: response.id,
            repo_owner: response.repo_owner,
            repo_name: response.repo_name,
            script: response.script,
            status: response.status.into(),
            created_at: response.created_at,
            updated_at: response.updated_at,
        }
    }
}
