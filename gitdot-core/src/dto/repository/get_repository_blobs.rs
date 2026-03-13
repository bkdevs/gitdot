use crate::{
    dto::{OwnerName, RepositoryBlobResponse, RepositoryName},
    error::RepositoryError,
};

#[derive(Debug, Clone)]
pub struct GetRepositoryBlobsRequest {
    pub name: RepositoryName,
    pub owner_name: OwnerName,
    pub ref_name: String,
    pub paths: Vec<String>,
}

impl GetRepositoryBlobsRequest {
    pub fn new(
        repo_name: &str,
        owner_name: &str,
        ref_name: String,
        paths: Vec<String>,
    ) -> Result<Self, RepositoryError> {
        Ok(Self {
            name: RepositoryName::try_new(repo_name)
                .map_err(|e| RepositoryError::InvalidRepositoryName(e.to_string()))?,
            owner_name: OwnerName::try_new(owner_name)
                .map_err(|e| RepositoryError::InvalidOwnerName(e.to_string()))?,
            ref_name,
            paths,
        })
    }
}

#[derive(Debug, Clone)]
pub struct RepositoryBlobsResponse {
    pub ref_name: String,
    pub commit_sha: String,
    pub blobs: Vec<RepositoryBlobResponse>,
}
