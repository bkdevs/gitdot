use chrono::{DateTime, Utc};

use crate::{
    dto::{OwnerName, RepositoryName, UserResponse},
    error::RepositoryError,
};

#[derive(Debug, Clone)]
pub struct GetRepositoryActivityRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
}

impl GetRepositoryActivityRequest {
    pub fn new(owner: &str, repo: &str) -> Result<Self, RepositoryError> {
        Ok(Self {
            owner: OwnerName::parse(owner, "owner name")?,
            repo: RepositoryName::parse(repo, "repository name")?,
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
