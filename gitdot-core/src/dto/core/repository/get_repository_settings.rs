use crate::{
    dto::{OwnerName, RepositoryName},
    error::{InputError, RepositoryError},
    model::CommitFilter,
};

pub struct GetRepositorySettingsRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
}

impl GetRepositorySettingsRequest {
    pub fn new(owner: &str, repo: &str) -> Result<Self, RepositoryError> {
        Ok(Self {
            owner: OwnerName::try_new(owner).map_err(|e| InputError::new("owner name", e))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| InputError::new("repository name", e))?,
        })
    }
}

pub struct RepositorySettingsResponse {
    pub commit_filters: Option<Vec<CommitFilter>>,
}
