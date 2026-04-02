use crate::{
    dto::{OwnerName, RepositoryName},
    error::{CommitError, InputError},
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
            owner: OwnerName::try_new(owner).map_err(|e| InputError::new("owner name", e))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| InputError::new("repository name", e))?,
            sha,
        })
    }
}
