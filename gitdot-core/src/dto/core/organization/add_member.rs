use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{
    dto::OwnerName,
    error::{InputError, OrganizationError},
    model::{OrganizationMember, OrganizationRole},
};

#[derive(Debug, Clone)]
pub struct AddMemberRequest {
    pub org_name: OwnerName,
    pub user_name: OwnerName,
    pub role: OrganizationRole,
}

impl AddMemberRequest {
    pub fn new(org_name: &str, user_name: &str, role: &str) -> Result<Self, OrganizationError> {
        let role = match role {
            "admin" => OrganizationRole::Admin,
            "member" => OrganizationRole::Member,
            _ => return Err(InputError::new("role", role).into()),
        };

        Ok(Self {
            org_name: OwnerName::try_new(org_name)
                .map_err(|e| InputError::new("organization name", e))?,
            user_name: OwnerName::try_new(user_name)
                .map_err(|e| InputError::new("user name", e))?,
            role,
        })
    }
}

#[derive(Debug, Clone)]
pub struct OrganizationMemberResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub organization_id: Uuid,
    pub role: OrganizationRole,
    pub created_at: DateTime<Utc>,
}

impl From<OrganizationMember> for OrganizationMemberResponse {
    fn from(member: OrganizationMember) -> Self {
        Self {
            id: member.id,
            user_id: member.user_id,
            organization_id: member.organization_id,
            role: member.role,
            created_at: member.created_at,
        }
    }
}
