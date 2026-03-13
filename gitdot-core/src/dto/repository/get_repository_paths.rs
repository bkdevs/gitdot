use crate::{
    dto::{OwnerName, RepositoryName},
    error::RepositoryError,
};

#[derive(Debug, Clone)]
pub struct GetRepositoryPathsRequest {
    pub name: RepositoryName,
    pub owner_name: OwnerName,
    pub ref_name: String,
}

impl GetRepositoryPathsRequest {
    pub fn new(
        repo_name: &str,
        owner_name: &str,
        ref_name: String,
    ) -> Result<Self, RepositoryError> {
        Ok(Self {
            name: RepositoryName::try_new(repo_name)
                .map_err(|e| RepositoryError::InvalidRepositoryName(e.to_string()))?,
            owner_name: OwnerName::try_new(owner_name)
                .map_err(|e| RepositoryError::InvalidOwnerName(e.to_string()))?,
            ref_name,
        })
    }
}

#[derive(Debug, Clone)]
pub struct RepositoryPathsResponse {
    pub ref_name: String,
    pub commit_sha: String,
    pub entries: Vec<RepositoryPath>,
}

#[derive(Debug, Clone)]
pub struct RepositoryPath {
    pub path: String,
    pub name: String,
    pub path_type: String,
    pub sha: String,
}
