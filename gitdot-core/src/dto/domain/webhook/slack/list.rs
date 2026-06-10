use crate::{
    dto::common::{OwnerName, RepositoryName},
    error::WebhookError,
    model::WebhookEventType,
};

#[derive(Debug, Clone)]
pub struct ListSlackWebhooksRequest {
    pub owner_name: OwnerName,
    pub repo_name: RepositoryName,
    pub event: WebhookEventType,
}

impl ListSlackWebhooksRequest {
    pub fn new(owner: &str, repo: &str, event: WebhookEventType) -> Result<Self, WebhookError> {
        Ok(Self {
            owner_name: OwnerName::parse(owner, "owner name")?,
            repo_name: RepositoryName::parse(repo, "repository name")?,
            event,
        })
    }
}
