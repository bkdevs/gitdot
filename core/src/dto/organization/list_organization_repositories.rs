use uuid::Uuid;

use crate::{dto::OwnerName, error::OrganizationError};

#[derive(Debug, Clone)]
pub struct ListOrganizationRepositoriesRequest {
    pub org_name: OwnerName,
    pub viewer_id: Option<Uuid>,
}

impl ListOrganizationRepositoriesRequest {
    pub fn new(org_name: &str, viewer_id: Option<Uuid>) -> Result<Self, OrganizationError> {
        Ok(Self {
            org_name: OwnerName::try_new(org_name)
                .map_err(|e| OrganizationError::InvalidOrganizationName(e.to_string()))?,
            viewer_id,
        })
    }
}
