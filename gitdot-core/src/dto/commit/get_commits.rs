use chrono::{DateTime, Utc};

use crate::{
    dto::{OwnerName, RepositoryName},
    error::CommitError,
};

#[derive(Debug, Clone)]
pub struct GetCommitsRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub ref_name: String,
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
}

impl GetCommitsRequest {
    pub fn new(
        owner: &str,
        repo: &str,
        ref_name: String,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<Self, CommitError> {
        Ok(Self {
            owner: OwnerName::try_new(owner)
                .map_err(|e| CommitError::InvalidOwnerName(e.to_string()))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| CommitError::InvalidRepositoryName(e.to_string()))?,
            ref_name,
            from,
            to,
        })
    }
}
