use crate::dto::{OwnerName, RepositoryName};
use crate::error::CommitError;

#[derive(Debug, Clone)]
pub struct CreateCommitsRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub old_sha: String,
    pub new_sha: String,
    pub ref_name: String,
}

impl CreateCommitsRequest {
    pub fn new(
        owner: &str,
        repo: &str,
        old_sha: String,
        new_sha: String,
        ref_name: String,
    ) -> Result<Self, CommitError> {
        Ok(Self {
            owner: OwnerName::try_new(owner)
                .map_err(|e| CommitError::InvalidOwnerName(e.to_string()))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| CommitError::InvalidOwnerName(e.to_string()))?,
            old_sha,
            new_sha,
            ref_name,
        })
    }
}
