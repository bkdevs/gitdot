use crate::{
    dto::{OwnerName, RepositoryName},
    error::RepositoryError,
    model::CommitFilter,
};

pub struct GetRepositorySettingsRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
}

impl GetRepositorySettingsRequest {
    pub fn new(owner: &str, repo: &str) -> Result<Self, RepositoryError> {
        Ok(Self {
            owner: OwnerName::try_new(owner)
                .map_err(|e| RepositoryError::InvalidOwnerName(e.to_string()))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| RepositoryError::InvalidRepositoryName(e.to_string()))?,
        })
    }
}

pub struct RepositorySettingsResponse {
    pub commit_filters: Option<Vec<CommitFilter>>,
}
