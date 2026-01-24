use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use gitdot_core::dto::OrganizationResponse;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CreateOrganizationServerResponse {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

impl From<OrganizationResponse> for CreateOrganizationServerResponse {
    fn from(org: OrganizationResponse) -> Self {
        Self {
            id: org.id,
            name: org.name,
            created_at: org.created_at,
        }
    }
}
