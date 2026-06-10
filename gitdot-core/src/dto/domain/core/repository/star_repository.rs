use uuid::Uuid;

use crate::{
    dto::{OwnerName, RepositoryName},
    error::{InputError, RepositoryError},
};

#[derive(Debug, Clone)]
pub struct StarRepositoryRequest {
    pub user_id: Uuid,
    pub owner: OwnerName,
    pub repo: RepositoryName,
}

impl StarRepositoryRequest {
    pub fn new(user_id: Uuid, owner: &str, repo: &str) -> Result<Self, RepositoryError> {
        Ok(Self {
            user_id,
            owner: OwnerName::parse(owner, "owner name")?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| InputError::new("repository name", e))?,
        })
    }
}
