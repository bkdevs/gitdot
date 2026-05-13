use chrono::{DateTime, Utc};

use crate::{
    dto::{OwnerName, RepositoryName, UserResponse},
    error::{InputError, RepositoryError},
};

#[derive(Debug, Clone)]
pub struct GetRepositoryActivityRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
}

impl GetRepositoryActivityRequest {
    pub fn new(owner: &str, repo: &str) -> Result<Self, RepositoryError> {
        Ok(Self {
            owner: OwnerName::try_new(owner).map_err(|e| InputError::new("owner name", e))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| InputError::new("repository name", e))?,
        })
    }
}

#[derive(Debug, Clone)]
pub enum RepositoryActivityEvent {
    Starred {
        user: UserResponse,
        at: DateTime<Utc>,
    },
}
