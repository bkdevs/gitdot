use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use gitdot_core::dto::OrganizationResponse;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CreateOrganizationServerResponse {
    id: Uuid,
    name: String,
    created_at: DateTime<Utc>,
}

impl From<OrganizationResponse> for CreateOrganizationServerResponse {
    fn from(response: OrganizationResponse) -> Self {
        Self {
            id: response.id,
            name: response.name,
            created_at: response.created_at,
        }
    }
}
