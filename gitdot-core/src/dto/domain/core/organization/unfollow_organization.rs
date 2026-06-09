use uuid::Uuid;

use crate::{
    dto::OwnerName,
    error::{InputError, OrganizationError},
};

#[derive(Debug, Clone)]
pub struct UnfollowOrganizationRequest {
    pub user_id: Uuid,
    pub org_name: OwnerName,
}

impl UnfollowOrganizationRequest {
    pub fn new(user_id: Uuid, org_name: &str) -> Result<Self, OrganizationError> {
        Ok(Self {
            user_id,
            org_name: OwnerName::try_new(org_name)
                .map_err(|e| InputError::new("organization name", e))?,
        })
    }
}
