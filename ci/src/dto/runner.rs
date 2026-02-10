use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use gitdot_core::dto::RunnerResponse;

#[derive(Debug, Clone, Deserialize)]
pub struct CreateRunnerServerRequest {
    pub name: String,
    pub owner_id: Uuid,
    pub owner_type: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RunnerServerResponse {
    pub id: Uuid,
    pub name: String,
    pub owner_id: Uuid,
    pub owner_type: String,
    pub created_at: DateTime<Utc>,
}

impl From<RunnerResponse> for RunnerServerResponse {
    fn from(response: RunnerResponse) -> Self {
        Self {
            id: response.id,
            name: response.name,
            owner_id: response.owner_id,
            owner_type: response.owner_type.into(),
            created_at: response.created_at,
        }
    }
}
