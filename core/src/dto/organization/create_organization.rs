use uuid::Uuid;

use super::OrganizationName;

#[derive(Debug, Clone)]
pub struct CreateOrganizationRequest {
    pub org_name: OrganizationName,
    pub owner_id: Uuid,
}

impl CreateOrganizationRequest {
    pub fn new(org_name: &str, owner_id: Uuid) -> Self {
        Self {
            org_name: OrganizationName::try_new(org_name).unwrap(),
            owner_id,
        }
    }
}
