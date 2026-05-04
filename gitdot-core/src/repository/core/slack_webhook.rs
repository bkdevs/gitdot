use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::DatabaseError,
    model::{SlackWebhook, WebhookEventType},
};

#[async_trait]
pub trait SlackWebhookRepository: Send + Sync + Clone + 'static {
    async fn create(
        &self,
        user_id: Uuid,
        repository_id: Uuid,
        events: &[WebhookEventType],
        slack_user_id: &str,
        slack_team_id: &str,
        slack_channel_id: &str,
    ) -> Result<SlackWebhook, DatabaseError>;

    async fn get(&self, id: Uuid) -> Result<Option<SlackWebhook>, DatabaseError>;

    async fn delete(&self, id: Uuid) -> Result<(), DatabaseError>;
}

#[derive(Debug, Clone)]
pub struct SlackWebhookRepositoryImpl {
    pool: PgPool,
}

impl SlackWebhookRepositoryImpl {
    pub fn new(pool: PgPool) -> SlackWebhookRepositoryImpl {
        SlackWebhookRepositoryImpl { pool }
    }
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl SlackWebhookRepository for SlackWebhookRepositoryImpl {
    async fn create(
        &self,
        user_id: Uuid,
        repository_id: Uuid,
        events: &[WebhookEventType],
        slack_user_id: &str,
        slack_team_id: &str,
        slack_channel_id: &str,
    ) -> Result<SlackWebhook, DatabaseError> {
        let webhook = sqlx::query_as::<_, SlackWebhook>(
            r#"
            INSERT INTO core.slack_webhooks
                (user_id, repository_id, events, slack_user_id, slack_team_id, slack_channel_id)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, user_id, repository_id, events,
                slack_user_id, slack_team_id, slack_channel_id, created_at
            "#,
        )
        .bind(user_id)
        .bind(repository_id)
        .bind(events)
        .bind(slack_user_id)
        .bind(slack_team_id)
        .bind(slack_channel_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(webhook)
    }

    async fn get(&self, id: Uuid) -> Result<Option<SlackWebhook>, DatabaseError> {
        let webhook = sqlx::query_as::<_, SlackWebhook>(
            r#"
            SELECT id, user_id, repository_id, events,
                slack_user_id, slack_team_id, slack_channel_id, created_at
            FROM core.slack_webhooks WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(webhook)
    }

    async fn delete(&self, id: Uuid) -> Result<(), DatabaseError> {
        sqlx::query("DELETE FROM core.slack_webhooks WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
