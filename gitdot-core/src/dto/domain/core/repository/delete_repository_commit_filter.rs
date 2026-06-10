use uuid::Uuid;

use crate::{
    dto::{OwnerName, RepositoryName},
    error::RepositoryError,
};

#[derive(Debug, Clone)]
pub struct DeleteRepositoryCommitFilterRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub filter_id: Uuid,
}

impl DeleteRepositoryCommitFilterRequest {
    pub fn new(owner: &str, repo: &str, filter_id: Uuid) -> Result<Self, RepositoryError> {
        Ok(Self {
            owner: OwnerName::parse(owner, "owner name")?,
            repo: RepositoryName::parse(repo, "repository name")?,
            filter_id,
        })
    }
}
