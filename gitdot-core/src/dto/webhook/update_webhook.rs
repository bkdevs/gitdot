use uuid::Uuid;

use crate::{
    dto::common::{OwnerName, RepositoryName, WebhookUrl},
    error::WebhookError,
    model::WebhookEventType,
};

#[derive(Debug, Clone)]
pub struct UpdateWebhookRequest {
    pub owner_name: OwnerName,
    pub repo_name: RepositoryName,
    pub webhook_id: Uuid,
    pub url: Option<WebhookUrl>,
    pub secret: Option<String>,
    pub events: Option<Vec<WebhookEventType>>,
}

impl UpdateWebhookRequest {
    pub fn new(
        owner: &str,
        repo: &str,
        webhook_id: Uuid,
        url: Option<&str>,
        secret: Option<String>,
        events: Option<Vec<WebhookEventType>>,
    ) -> Result<Self, WebhookError> {
        let url = url
            .map(WebhookUrl::try_new)
            .transpose()
            .map_err(|e| WebhookError::InvalidUrl(e.to_string()))?;

        Ok(Self {
            owner_name: OwnerName::try_new(owner)
                .map_err(|e| WebhookError::InvalidOwnerName(e.to_string()))?,
            repo_name: RepositoryName::try_new(repo)
                .map_err(|e| WebhookError::InvalidRepositoryName(e.to_string()))?,
            webhook_id,
            url,
            secret,
            events,
        })
    }
}
