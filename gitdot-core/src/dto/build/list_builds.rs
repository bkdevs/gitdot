use crate::{
    dto::common::{OwnerName, RepositoryName},
    error::BuildError,
};

#[derive(Debug, Clone)]
pub struct ListBuildsRequest {
    pub repo_owner: OwnerName,
    pub repo_name: RepositoryName,
}

impl ListBuildsRequest {
    pub fn new(repo_owner: &str, repo_name: &str) -> Result<Self, BuildError> {
        Ok(Self {
            repo_owner: OwnerName::try_new(repo_owner)
                .map_err(|e| BuildError::InvalidOwnerName(e.to_string()))?,
            repo_name: RepositoryName::try_new(repo_name)
                .map_err(|e| BuildError::InvalidRepositoryName(e.to_string()))?,
        })
    }
}
