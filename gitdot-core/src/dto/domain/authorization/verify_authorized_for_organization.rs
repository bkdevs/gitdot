use uuid::Uuid;

use crate::{dto::OwnerName, error::AuthorizationError};

#[derive(Debug, Clone)]
pub struct OrganizationAuthorizationRequest {
    pub user_id: Uuid,
    pub org_name: OwnerName,
}

impl OrganizationAuthorizationRequest {
    pub fn new(user_id: Uuid, org_name: &str) -> Result<Self, AuthorizationError> {
        Ok(Self {
            user_id,
            org_name: OwnerName::parse(org_name, "organization name")?,
        })
    }
}
