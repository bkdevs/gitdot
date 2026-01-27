use uuid::Uuid;

use crate::dto::OwnerName;
use crate::error::AuthorizationError;
use crate::model::RepositoryOwnerType;

#[derive(Debug, Clone)]
pub struct RepositoryCreationAuthorizationRequest {
    pub user_id: Uuid,
    pub owner: OwnerName,
    pub owner_type: RepositoryOwnerType,
}

impl RepositoryCreationAuthorizationRequest {
    pub fn new(user_id: Uuid, owner: &str, owner_type: &str) -> Result<Self, AuthorizationError> {
        Ok(Self {
            user_id,
            owner: OwnerName::try_new(owner)
                .map_err(|e| AuthorizationError::InvalidRequest(e.to_string()))?,
            owner_type: RepositoryOwnerType::try_from(owner_type)
                .map_err(|e| AuthorizationError::InvalidRequest(e.to_string()))?,
        })
    }
}
