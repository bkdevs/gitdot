use crate::{
    dto::{OwnerName, RepositoryName},
    error::{InputError, RepositoryError},
};

#[derive(Debug, Clone)]
pub struct DeleteRepositoryRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
}

impl DeleteRepositoryRequest {
    pub fn new(owner: &str, repo: &str) -> Result<Self, RepositoryError> {
        Ok(Self {
            owner: OwnerName::try_new(owner).map_err(|e| InputError::new("owner name", e))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| InputError::new("repository name", e))?,
        })
    }
}
