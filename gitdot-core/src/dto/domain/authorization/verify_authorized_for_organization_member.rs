use uuid::Uuid;

use crate::{dto::OwnerName, error::AuthorizationError};

#[derive(Debug, Clone)]
pub struct OrganizationMemberAuthorizationRequest {
    pub auth_user_id: Uuid,
    pub org_name: OwnerName,
    pub member_id: Uuid,
}

impl OrganizationMemberAuthorizationRequest {
    pub fn new(
        auth_user_id: Uuid,
        org_name: &str,
        member_id: Uuid,
    ) -> Result<Self, AuthorizationError> {
        Ok(Self {
            auth_user_id,
            org_name: OwnerName::parse(org_name, "organization name")?,
            member_id,
        })
    }
}
