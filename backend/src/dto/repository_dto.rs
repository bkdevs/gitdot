use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use gitdot_core::dto::RepositoryResponse;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CreateRepositoryRequest {
    pub owner_type: String,

    #[serde(default = "default_visibility")]
    pub visibility: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CreateRepositoryResponse {
    pub id: Uuid,
    pub name: String,
    pub owner_name: String,
    pub visibility: String,
    pub created_at: DateTime<Utc>,
}

impl From<&RepositoryResponse> for CreateRepositoryResponse {
    fn from(repo: &RepositoryResponse) -> Self {
        Self {
            id: repo.id,
            name: repo.name.clone(),
            owner_name: repo.owner.clone(),
            visibility: repo.visibility.clone(),
            created_at: repo.created_at,
        }
    }
}

fn default_visibility() -> String {
    "public".to_string()
}
