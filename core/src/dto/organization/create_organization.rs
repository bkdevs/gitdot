use uuid::Uuid;

use crate::dto::OwnerName;
use crate::error::OrganizationError;

#[derive(Debug, Clone)]
pub struct CreateOrganizationRequest {
    pub org_name: OwnerName,
    pub owner_id: Uuid,
}

impl CreateOrganizationRequest {
    pub fn new(org_name: &str, owner_id: Uuid) -> Result<Self, OrganizationError> {
        Ok(Self {
            org_name: OwnerName::try_new(org_name)
                .map_err(|e| OrganizationError::InvalidOrganizationName(e.to_string()))?,
            owner_id,
        })
    }
}
