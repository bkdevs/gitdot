use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use gitdot_core::dto::OrganizationResponse;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CreateOrganizationResponse {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

impl From<OrganizationResponse> for CreateOrganizationResponse {
    fn from(org: OrganizationResponse) -> Self {
        Self {
            id: org.id,
            name: org.name.clone(),
            created_at: org.created_at,
        }
    }
}
