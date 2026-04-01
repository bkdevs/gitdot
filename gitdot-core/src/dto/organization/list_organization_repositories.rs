use uuid::Uuid;

use crate::{
    dto::OwnerName,
    error::{InputError, OrganizationError},
};

#[derive(Debug, Clone)]
pub struct ListOrganizationRepositoriesRequest {
    pub org_name: OwnerName,
    pub viewer_id: Option<Uuid>,
}

impl ListOrganizationRepositoriesRequest {
    pub fn new(org_name: &str, viewer_id: Option<Uuid>) -> Result<Self, OrganizationError> {
        Ok(Self {
            org_name: OwnerName::try_new(org_name)
                .map_err(|e| InputError::new("organization name", e))?,
            viewer_id,
        })
    }
}
