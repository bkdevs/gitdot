use crate::{dto::OwnerName, error::OrganizationError};

#[derive(Debug, Clone)]
pub struct GetOrganizationRequest {
    pub org_name: OwnerName,
}

impl GetOrganizationRequest {
    pub fn new(org_name: &str) -> Result<Self, OrganizationError> {
        Ok(Self {
            org_name: OwnerName::parse(org_name, "organization name")?,
        })
    }
}
