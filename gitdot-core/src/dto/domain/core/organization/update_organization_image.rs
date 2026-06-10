use bytes::Bytes;

use crate::{dto::OwnerName, error::OrganizationError};

#[derive(Debug, Clone)]
pub struct UpdateOrganizationImageRequest {
    pub org_name: OwnerName,
    pub bytes: Bytes,
}

impl UpdateOrganizationImageRequest {
    pub fn new(org_name: &str, bytes: Bytes) -> Result<Self, OrganizationError> {
        Ok(Self {
            org_name: OwnerName::parse(org_name, "organization name")?,
            bytes,
        })
    }
}
