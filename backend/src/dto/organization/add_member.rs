use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use gitdot_core::dto::OrganizationMemberResponse;
use gitdot_core::model::OrganizationRole;

#[derive(Debug, Clone, Deserialize)]
pub struct AddMemberServerRequest {
    pub user_name: String,
    pub role: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AddMemberServerResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub organization_id: Uuid,
    pub role: String,
    pub created_at: DateTime<Utc>,
}

impl From<OrganizationMemberResponse> for AddMemberServerResponse {
    fn from(member: OrganizationMemberResponse) -> Self {
        Self {
            id: member.id,
            user_id: member.user_id,
            organization_id: member.organization_id,
            role: match member.role {
                OrganizationRole::Admin => "admin".to_string(),
                OrganizationRole::Member => "member".to_string(),
            },
            created_at: member.created_at,
        }
    }
}
