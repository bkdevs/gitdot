use crate::{
    dto::{OwnerName, RepositoryName},
    error::{InputError, RepositoryError},
};

pub struct GetRepositoryRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
}

impl GetRepositoryRequest {
    pub fn new(owner: &str, repo: &str) -> Result<Self, RepositoryError> {
        Ok(Self {
            owner: OwnerName::try_new(owner).map_err(|e| InputError::new("owner name", e))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| InputError::new("repository name", e))?,
        })
    }
}
