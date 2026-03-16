use crate::{
    dto::{OwnerName, RepositoryName},
    error::CommitError,
};

#[derive(Debug, Clone)]
pub struct GetCommitDiffRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub sha: String,
}

impl GetCommitDiffRequest {
    pub fn new(owner: &str, repo: &str, sha: String) -> Result<Self, CommitError> {
        Ok(Self {
            owner: OwnerName::try_new(owner)
                .map_err(|e| CommitError::InvalidOwnerName(e.to_string()))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| CommitError::InvalidRepositoryName(e.to_string()))?,
            sha,
        })
    }
}
