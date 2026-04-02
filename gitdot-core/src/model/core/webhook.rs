use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct Webhook {
    pub id: Uuid,
    pub repository_id: Uuid,
    pub url: String,
    pub secret: String,
    pub events: Vec<WebhookEventType>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Type, Serialize, Deserialize)]
#[sqlx(type_name = "core.webhook_event_type", rename_all = "snake_case")]
pub enum WebhookEventType {
    Push,
}

impl Into<String> for WebhookEventType {
    fn into(self) -> String {
        match self {
            WebhookEventType::Push => "push".to_string(),
        }
    }
}

impl TryFrom<&str> for WebhookEventType {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "push" => Ok(WebhookEventType::Push),
            _ => Err(format!("Invalid webhook event type: {value}")),
        }
    }
}
