use uuid::Uuid;

use crate::dto::{OwnerName, RepositoryName};
use crate::error::AuthorizationError;

#[derive(Debug, Clone)]
pub struct RepositoryAuthorizationRequest {
    pub user_id: Option<Uuid>,
    pub owner: OwnerName,
    pub repo: RepositoryName,
}

impl RepositoryAuthorizationRequest {
    pub fn new(user_id: Option<Uuid>, owner: &str, repo: &str) -> Result<Self, AuthorizationError> {
        Ok(Self {
            user_id: user_id,
            owner: OwnerName::try_new(owner)
                .map_err(|e| AuthorizationError::InvalidRequest(e.to_string()))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| AuthorizationError::InvalidRequest(e.to_string()))?,
        })
    }

    pub fn get_repo_path(&self) -> String {
        format!("{}/{}", self.owner.as_ref(), self.repo.as_ref())
    }
}
