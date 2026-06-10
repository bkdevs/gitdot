use uuid::Uuid;

use crate::{dto::OwnerName, error::OrganizationError};

#[derive(Debug, Clone)]
pub struct UpdateOrganizationMemberRequest {
    pub org_name: OwnerName,
    pub member_id: Uuid,
    pub role_description: Option<String>,
}

impl UpdateOrganizationMemberRequest {
    pub fn new(
        org_name: &str,
        member_id: Uuid,
        role_description: Option<String>,
    ) -> Result<Self, OrganizationError> {
        Ok(Self {
            org_name: OwnerName::parse(org_name, "organization name")?,
            member_id,
            role_description,
        })
    }
}
