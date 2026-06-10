use uuid::Uuid;

use crate::{
    dto::{OwnerName, RepositoryName},
    error::{InputError, RepositoryError},
};

pub struct GetRepositoryRequest {
    pub user_id: Option<Uuid>,
    pub owner: OwnerName,
    pub repo: RepositoryName,
}

impl GetRepositoryRequest {
    pub fn new(user_id: Option<Uuid>, owner: &str, repo: &str) -> Result<Self, RepositoryError> {
        Ok(Self {
            user_id,
            owner: OwnerName::parse(owner, "owner name")?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| InputError::new("repository name", e))?,
        })
    }
}
