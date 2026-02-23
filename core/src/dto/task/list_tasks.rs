use crate::{
    dto::common::{OwnerName, RepositoryName},
    error::TaskError,
};

#[derive(Debug, Clone)]
pub struct ListTasksRequest {
    pub repo_owner: OwnerName,
    pub repo_name: RepositoryName,
}

impl ListTasksRequest {
    pub fn new(repo_owner: &str, repo_name: &str) -> Result<Self, TaskError> {
        Ok(Self {
            repo_owner: OwnerName::try_new(repo_owner)
                .map_err(|e| TaskError::InvalidOwnerName(e.to_string()))?,
            repo_name: RepositoryName::try_new(repo_name)
                .map_err(|e| TaskError::InvalidRepositoryName(e.to_string()))?,
        })
    }
}
