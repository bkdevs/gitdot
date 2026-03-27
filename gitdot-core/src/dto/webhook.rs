mod create_webhook;
mod delete_webhook;
mod get_webhook;
mod list_webhooks;
mod update_webhook;

use chrono::{DateTime, Utc};
use uuid::Uuid;

pub use create_webhook::CreateWebhookRequest;
pub use delete_webhook::DeleteWebhookRequest;
pub use get_webhook::GetWebhookRequest;
pub use list_webhooks::ListWebhooksRequest;
pub use update_webhook::UpdateWebhookRequest;

use crate::model::{Webhook, WebhookEventType};

#[derive(Debug, Clone)]
pub struct WebhookResponse {
    pub id: Uuid,
    pub repository_id: Uuid,
    pub url: String,
    pub secret: String,
    pub events: Vec<WebhookEventType>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Webhook> for WebhookResponse {
    fn from(webhook: Webhook) -> Self {
        Self {
            id: webhook.id,
            repository_id: webhook.repository_id,
            url: webhook.url,
            secret: webhook.secret,
            events: webhook.events,
            created_at: webhook.created_at,
            updated_at: webhook.updated_at,
        }
    }
}
