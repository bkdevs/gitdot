use crate::{
    dto::{OwnerName, RepositoryName},
    error::{InputError, RepositoryError},
};

#[derive(Debug, Clone)]
pub struct GetRepositoryCommitRequest {
    pub name: RepositoryName,
    pub owner_name: OwnerName,
    pub ref_name: String,
}

impl GetRepositoryCommitRequest {
    pub fn new(repo_name: &str, owner_name: &str, sha: String) -> Result<Self, RepositoryError> {
        Ok(Self {
            name: RepositoryName::try_new(repo_name)
                .map_err(|e| InputError::new("repository name", e))?,
            owner_name: OwnerName::try_new(owner_name)
                .map_err(|e| InputError::new("owner name", e))?,
            ref_name: sha,
        })
    }
}
