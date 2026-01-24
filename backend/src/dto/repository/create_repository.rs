use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use gitdot_core::dto::RepositoryResponse;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CreateRepositoryServerRequest {
    pub owner_type: String,

    #[serde(default = "default_visibility")]
    pub visibility: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CreateRepositoryServerResponse {
    id: Uuid,
    name: String,
    owner_name: String,
    visibility: String,
    created_at: DateTime<Utc>,
}

impl From<RepositoryResponse> for CreateRepositoryServerResponse {
    fn from(response: RepositoryResponse) -> Self {
        Self {
            id: response.id,
            name: response.name,
            owner_name: response.owner,
            visibility: response.visibility,
            created_at: response.created_at,
        }
    }
}

fn default_visibility() -> String {
    "public".to_string()
}
