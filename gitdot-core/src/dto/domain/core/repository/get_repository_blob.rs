use crate::{
    dto::{OwnerName, RepositoryName},
    error::RepositoryError,
};

#[derive(Debug, Clone)]
pub struct GetRepositoryBlobRequest {
    pub name: RepositoryName,
    pub owner_name: OwnerName,
    pub ref_name: String,
    pub path: String,
}

impl GetRepositoryBlobRequest {
    pub fn new(
        repo_name: &str,
        owner_name: &str,
        ref_name: String,
        path: String,
    ) -> Result<Self, RepositoryError> {
        Ok(Self {
            name: RepositoryName::parse(repo_name, "repository name")?,
            owner_name: OwnerName::parse(owner_name, "owner name")?,
            ref_name,
            path,
        })
    }
}

#[derive(Debug, Clone)]
pub struct RepositoryBlobResponse {
    pub commit_sha: String,
    pub path: String,
    pub sha: String,
    pub content: String,
    pub encoding: String,
}
