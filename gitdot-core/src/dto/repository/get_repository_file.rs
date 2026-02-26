use crate::{
    dto::{OwnerName, RepositoryName},
    error::RepositoryError,
};

#[derive(Debug, Clone)]
pub struct GetRepositoryFileRequest {
    pub name: RepositoryName,
    pub owner_name: OwnerName,
    pub ref_name: String,
    pub path: String,
}

impl GetRepositoryFileRequest {
    pub fn new(
        repo_name: &str,
        owner_name: &str,
        ref_name: String,
        path: String,
    ) -> Result<Self, RepositoryError> {
        Ok(Self {
            name: RepositoryName::try_new(repo_name)
                .map_err(|e| RepositoryError::InvalidRepositoryName(e.to_string()))?,
            owner_name: OwnerName::try_new(owner_name)
                .map_err(|e| RepositoryError::InvalidOwnerName(e.to_string()))?,
            ref_name,
            path,
        })
    }
}

#[derive(Debug, Clone)]
pub struct RepositoryFileResponse {
    pub ref_name: String,
    pub path: String,
    pub commit_sha: String,
    pub sha: String,
    pub content: String,
    pub encoding: String,
}
