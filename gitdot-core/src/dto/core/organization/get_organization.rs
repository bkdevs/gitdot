use crate::{
    dto::OwnerName,
    error::{InputError, OrganizationError},
};

#[derive(Debug, Clone)]
pub struct GetOrganizationRequest {
    pub org_name: OwnerName,
}

impl GetOrganizationRequest {
    pub fn new(org_name: &str) -> Result<Self, OrganizationError> {
        Ok(Self {
            org_name: OwnerName::try_new(org_name)
                .map_err(|e| InputError::new("organization name", e))?,
        })
    }
}
