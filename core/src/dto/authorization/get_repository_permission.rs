use uuid::Uuid;

use crate::{
    dto::{OwnerName, RepositoryName},
    error::AuthorizationError,
};

use super::RepositoryPermission;

#[derive(Debug, Clone)]
pub struct GetRepositoryPermissionRequest {
    pub user_id: Uuid,
    pub owner: OwnerName,
    pub repo: RepositoryName,
}

impl GetRepositoryPermissionRequest {
    pub fn new(user_id: Uuid, owner: &str, repo: &str) -> Result<Self, AuthorizationError> {
        Ok(Self {
            user_id,
            owner: OwnerName::try_new(owner)
                .map_err(|e| AuthorizationError::InvalidRequest(e.to_string()))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| AuthorizationError::InvalidRequest(e.to_string()))?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetRepositoryPermissionResponse {
    pub permission: RepositoryPermission,
}
