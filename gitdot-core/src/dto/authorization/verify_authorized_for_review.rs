use uuid::Uuid;

use crate::{
    dto::{OwnerName, RepositoryName, ReviewId},
    error::{AuthorizationError, InputError},
};

#[derive(Debug, Clone)]
pub struct ReviewAuthorizationRequest {
    pub user_id: Uuid,
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub review_id: ReviewId,
}

impl ReviewAuthorizationRequest {
    pub fn new(
        user_id: Uuid,
        owner_name: &str,
        repo_name: &str,
        review_id: ReviewId,
    ) -> Result<Self, AuthorizationError> {
        Ok(Self {
            user_id,
            owner: OwnerName::try_new(owner_name).map_err(|e| InputError::new("owner name", e))?,
            repo: RepositoryName::try_new(repo_name)
                .map_err(|e| InputError::new("repository name", e))?,
            review_id,
        })
    }
}
