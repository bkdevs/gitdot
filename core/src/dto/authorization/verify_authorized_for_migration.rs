use uuid::Uuid;

use crate::{dto::OwnerName, error::AuthorizationError};

#[derive(Debug, Clone)]
pub struct MigrationAuthorizationRequest {
    pub user_id: Uuid,
    pub owner_name: OwnerName,
}

impl MigrationAuthorizationRequest {
    pub fn new(user_id: Uuid, owner_name: &str) -> Result<Self, AuthorizationError> {
        Ok(Self {
            user_id,
            owner_name: OwnerName::try_new(owner_name)
                .map_err(|e| AuthorizationError::InvalidRequest(e.to_string()))?,
        })
    }
}
