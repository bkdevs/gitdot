use uuid::Uuid;

use crate::{
    dto::common::{OwnerName, RepositoryName, WebhookUrl},
    error::{InputError, WebhookError},
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
        events: Option<Vec<String>>,
    ) -> Result<Self, WebhookError> {
        let url = url
            .map(WebhookUrl::try_new)
            .transpose()
            .map_err(|e| InputError::new("url", e))?;

        let events = events
            .map(|evts| {
                evts.iter()
                    .map(|e| {
                        WebhookEventType::try_from(e.as_str())
                            .map_err(|e| InputError::new("event type", e))
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?;

        Ok(Self {
            owner_name: OwnerName::parse(owner, "owner name")?,
            repo_name: RepositoryName::parse(repo, "repository name")?,
            webhook_id,
            url,
            secret,
            events,
        })
    }
}
