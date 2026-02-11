use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use gitdot_core::dto::DagResponse;

#[derive(Debug, Clone, Deserialize)]
pub struct CreateDagServerRequest {
    pub repo_owner: String,
    pub repo_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DagServerResponse {
    pub id: Uuid,
    pub repo_owner: String,
    pub repo_name: String,
    pub task_ids: Vec<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<DagResponse> for DagServerResponse {
    fn from(response: DagResponse) -> Self {
        Self {
            id: response.id,
            repo_owner: response.repo_owner,
            repo_name: response.repo_name,
            task_ids: response.task_ids,
            created_at: response.created_at,
            updated_at: response.updated_at,
        }
    }
}
