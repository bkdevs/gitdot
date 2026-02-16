use crate::{
    dto::{OwnerName, RepositoryName},
    error::RepositoryError,
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
                .map_err(|e| RepositoryError::InvalidRepositoryName(e.to_string()))?,
            owner_name: OwnerName::try_new(owner_name)
                .map_err(|e| RepositoryError::InvalidOwnerName(e.to_string()))?,
            ref_name: sha,
        })
    }
}
