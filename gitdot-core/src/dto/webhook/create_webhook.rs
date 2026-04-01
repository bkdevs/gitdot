use crate::{
    dto::common::{OwnerName, RepositoryName, WebhookUrl},
    error::{InputError, WebhookError},
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
            return Err(InputError::new("secret", "secret cannot be empty").into());
        }

        if events.is_empty() {
            return Err(InputError::new("event type", "at least one event is required").into());
        }

        let events = events
            .iter()
            .map(|e| {
                WebhookEventType::try_from(e.as_str()).map_err(|e| InputError::new("event type", e))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            owner_name: OwnerName::try_new(owner).map_err(|e| InputError::new("owner name", e))?,
            repo_name: RepositoryName::try_new(repo)
                .map_err(|e| InputError::new("repository name", e))?,
            url: WebhookUrl::try_new(url).map_err(|e| InputError::new("url", e))?,
            secret,
            events,
        })
    }
}
