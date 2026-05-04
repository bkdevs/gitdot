mod create_webhook;
mod delete_webhook;
mod event;
mod get_webhook;
mod list_webhooks;
mod slack;
mod update_webhook;

use chrono::{DateTime, Utc};
use uuid::Uuid;

pub use create_webhook::CreateWebhookRequest;
pub use delete_webhook::DeleteWebhookRequest;
pub use event::*;
pub use get_webhook::GetWebhookRequest;
pub use list_webhooks::ListWebhooksRequest;
pub use slack::*;
pub use update_webhook::UpdateWebhookRequest;

use crate::model::{SlackWebhook, Webhook, WebhookEventType};

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

#[derive(Debug, Clone)]
pub struct SlackWebhookResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub repository_id: Uuid,
    pub events: Vec<WebhookEventType>,
    pub slack_user_id: String,
    pub slack_team_id: String,
    pub slack_channel_id: String,
    pub created_at: DateTime<Utc>,
}

impl From<SlackWebhook> for SlackWebhookResponse {
    fn from(webhook: SlackWebhook) -> Self {
        Self {
            id: webhook.id,
            user_id: webhook.user_id,
            repository_id: webhook.repository_id,
            events: webhook.events,
            slack_user_id: webhook.slack_user_id,
            slack_team_id: webhook.slack_team_id,
            slack_channel_id: webhook.slack_channel_id,
            created_at: webhook.created_at,
        }
    }
}
