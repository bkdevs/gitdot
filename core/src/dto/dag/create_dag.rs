use std::collections::HashMap;

use uuid::Uuid;

use crate::{
    dto::common::{OwnerName, RepositoryName},
    error::DagError,
};

#[derive(Debug, Clone)]
pub struct CreateDagRequest {
    pub repo_owner: OwnerName,
    pub repo_name: RepositoryName,
    pub task_dependencies: HashMap<Uuid, Vec<Uuid>>,
}

impl CreateDagRequest {
    pub fn new(
        repo_owner: &str,
        repo_name: &str,
        task_dependencies: HashMap<Uuid, Vec<Uuid>>,
    ) -> Result<Self, DagError> {
        Ok(Self {
            repo_owner: OwnerName::try_new(repo_owner)
                .map_err(|e| DagError::InvalidOwnerName(e.to_string()))?,
            repo_name: RepositoryName::try_new(repo_name)
                .map_err(|e| DagError::InvalidRepositoryName(e.to_string()))?,
            task_dependencies,
        })
    }
}
