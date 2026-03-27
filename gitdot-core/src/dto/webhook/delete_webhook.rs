use uuid::Uuid;

use crate::{
    dto::common::{OwnerName, RepositoryName},
    error::WebhookError,
};

#[derive(Debug, Clone)]
pub struct DeleteWebhookRequest {
    pub owner_name: OwnerName,
    pub repo_name: RepositoryName,
    pub webhook_id: Uuid,
}

impl DeleteWebhookRequest {
    pub fn new(owner: &str, repo: &str, webhook_id: Uuid) -> Result<Self, WebhookError> {
        Ok(Self {
            owner_name: OwnerName::try_new(owner)
                .map_err(|e| WebhookError::InvalidOwnerName(e.to_string()))?,
            repo_name: RepositoryName::try_new(repo)
                .map_err(|e| WebhookError::InvalidRepositoryName(e.to_string()))?,
            webhook_id,
        })
    }
}
