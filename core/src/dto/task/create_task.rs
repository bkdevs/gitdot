use uuid::Uuid;

use crate::{
    dto::common::{OwnerName, RepositoryName},
    error::TaskError,
};

#[derive(Debug, Clone)]
pub struct CreateTaskRequest {
    pub repo_owner: OwnerName,
    pub repo_name: RepositoryName,
    pub script: String,
    pub dag_id: Uuid,
    pub user_id: Uuid,
}

impl CreateTaskRequest {
    pub fn new(
        repo_owner: &str,
        repo_name: &str,
        script: String,
        dag_id: Uuid,
        user_id: Uuid,
    ) -> Result<Self, TaskError> {
        Ok(Self {
            repo_owner: OwnerName::try_new(repo_owner)
                .map_err(|e| TaskError::InvalidOwnerName(e.to_string()))?,
            repo_name: RepositoryName::try_new(repo_name)
                .map_err(|e| TaskError::InvalidRepositoryName(e.to_string()))?,
            script,
            dag_id,
            user_id,
        })
    }
}
