use uuid::Uuid;

use crate::dto::{OwnerName, RepositoryName};
use crate::error::AuthorizationError;

#[derive(Debug, Clone)]
pub struct RepositoryAuthorizationRequest {
    pub user_id: Option<Uuid>,
    pub owner_name: OwnerName,
    pub repo_name: RepositoryName,
}

impl RepositoryAuthorizationRequest {
    pub fn new(
        user_id: Option<Uuid>,
        owner_name: &str,
        repo_name: &str,
    ) -> Result<Self, AuthorizationError> {
        Ok(Self {
            user_id: user_id,
            owner_name: OwnerName::try_new(owner_name)
                .map_err(|e| AuthorizationError::InvalidRequest(e.to_string()))?,
            repo_name: RepositoryName::try_new(repo_name)
                .map_err(|e| AuthorizationError::InvalidRequest(e.to_string()))?,
        })
    }

    pub fn get_repo_path(&self) -> String {
        format!(
            "{}/{}",
            &self.owner_name.to_string(),
            &self.repo_name.to_string()
        )
    }
}
