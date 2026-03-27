use crate::{
    dto::common::{OwnerName, RepositoryName, WebhookUrl},
    error::WebhookError,
    model::WebhookEventType,
};

#[derive(Debug, Clone)]
pub struct CreateWebhookRequest {
    pub owner_name: OwnerName,
    pub repo_name: RepositoryName,
    pub url: WebhookUrl,
    pub secret: String,
    pub events: Vec<WebhookEventType>,
}

impl CreateWebhookRequest {
    pub fn new(
        owner: &str,
        repo: &str,
        url: &str,
        secret: String,
        events: Vec<String>,
    ) -> Result<Self, WebhookError> {
        if secret.is_empty() {
            return Err(WebhookError::InvalidSecret(
                "secret cannot be empty".to_string(),
            ));
        }

        if events.is_empty() {
            return Err(WebhookError::InvalidEventType(
                "at least one event is required".to_string(),
            ));
        }

        let events = events
            .iter()
            .map(|e| WebhookEventType::try_from(e.as_str()).map_err(WebhookError::InvalidEventType))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            owner_name: OwnerName::try_new(owner)
                .map_err(|e| WebhookError::InvalidOwnerName(e.to_string()))?,
            repo_name: RepositoryName::try_new(repo)
                .map_err(|e| WebhookError::InvalidRepositoryName(e.to_string()))?,
            url: WebhookUrl::try_new(url).map_err(|e| WebhookError::InvalidUrl(e.to_string()))?,
            secret,
            events,
        })
    }
}
