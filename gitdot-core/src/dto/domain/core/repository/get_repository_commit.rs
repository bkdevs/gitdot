use crate::{
    dto::{OwnerName, RepositoryName},
    error::{CommitError, InputError},
};

#[derive(Debug, Clone)]
pub struct GetRepositoryCommitRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub sha: String,
}

impl GetRepositoryCommitRequest {
    pub fn new(owner: &str, repo: &str, sha: String) -> Result<Self, CommitError> {
        Ok(Self {
            owner: OwnerName::parse(owner, "owner name")?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| InputError::new("repository name", e))?,
            sha,
        })
    }
}
