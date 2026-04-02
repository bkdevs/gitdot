use crate::{
    dto::{OwnerName, RepositoryName},
    error::{InputError, RepositoryError},
    model::CommitFilter,
};

pub struct UpdateRepositorySettingsRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub commit_filters: Option<Vec<CommitFilter>>,
}

impl UpdateRepositorySettingsRequest {
    pub fn new(
        owner: &str,
        repo: &str,
        commit_filters: Option<Vec<CommitFilter>>,
    ) -> Result<Self, RepositoryError> {
        Ok(Self {
            owner: OwnerName::try_new(owner).map_err(|e| InputError::new("owner name", e))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| InputError::new("repository name", e))?,
            commit_filters,
        })
    }
}
