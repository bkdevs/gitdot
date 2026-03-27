use crate::{
    dto::{OwnerName, RepositoryBlobResponse, RepositoryName},
    error::RepositoryError,
};

#[derive(Debug, Clone)]
pub struct GetRepositoryBlobsRequest {
    pub name: RepositoryName,
    pub owner_name: OwnerName,
    pub refs: Vec<String>,
    pub paths: Vec<String>,
}

impl GetRepositoryBlobsRequest {
    pub fn new(
        repo_name: &str,
        owner_name: &str,
        refs: Vec<String>,
        paths: Vec<String>,
    ) -> Result<Self, RepositoryError> {
        if refs.len() > 1 && paths.len() != 1 {
            return Err(RepositoryError::TooManyPaths);
        }
        Ok(Self {
            name: RepositoryName::try_new(repo_name)
                .map_err(|e| RepositoryError::InvalidRepositoryName(e.to_string()))?,
            owner_name: OwnerName::try_new(owner_name)
                .map_err(|e| RepositoryError::InvalidOwnerName(e.to_string()))?,
            refs,
            paths,
        })
    }
}

#[derive(Debug, Clone)]
pub struct RepositoryBlobsResponse {
    pub blobs: Vec<RepositoryBlobResponse>,
}
