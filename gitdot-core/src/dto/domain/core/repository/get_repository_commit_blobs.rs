use crate::{
    dto::{OwnerName, RepositoryName},
    error::CommitError,
};

#[derive(Debug, Clone)]
pub struct GetRepositoryCommitBlobsRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub sha: String,
}

impl GetRepositoryCommitBlobsRequest {
    pub fn new(owner: &str, repo: &str, sha: String) -> Result<Self, CommitError> {
        Ok(Self {
            owner: OwnerName::parse(owner, "owner name")?,
            repo: RepositoryName::parse(repo, "repository name")?,
            sha,
        })
    }
}
