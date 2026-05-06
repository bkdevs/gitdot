use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    dto::{OwnerName, RepositoryName},
    error::{InputError, WebhookError},
};

#[derive(Debug, Clone)]
pub struct PublishRepoPushRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub ref_name: String,
    pub old_sha: String,
    pub new_sha: String,
    pub pusher_id: Uuid,
}

impl PublishRepoPushRequest {
    pub fn new(
        owner: &str,
        repo: &str,
        ref_name: String,
        old_sha: String,
        new_sha: String,
        pusher_id: Uuid,
    ) -> Result<Self, WebhookError> {
        Ok(Self {
            owner: OwnerName::try_new(owner).map_err(|e| InputError::new("owner name", e))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| InputError::new("repository name", e))?,
            ref_name,
            old_sha,
            new_sha,
            pusher_id,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoPushEvent {
    pub owner: String,
    pub repo: String,
    pub ref_name: String,
    pub old_sha: String,
    pub new_sha: String,
    pub pusher_id: Uuid,
    pub pusher_name: String,
    pub commits: Vec<RepoPushCommit>,
    pub pushed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoPushCommit {
    pub sha: String,
    pub message: String,
}
