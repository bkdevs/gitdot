use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use gitdot_core::models::Organization;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CreateOrganizationResponse {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

impl From<&Organization> for CreateOrganizationResponse {
    fn from(org: &Organization) -> Self {
        Self {
            id: org.id,
            name: org.name.clone(),
            created_at: org.created_at,
        }
    }
}
