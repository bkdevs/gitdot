use async_trait::async_trait;
use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::model::{Webhook, WebhookEventType};

#[async_trait]
pub trait WebhookRepository: Send + Sync + Clone + 'static {
    async fn create(
        &self,
        repository_id: Uuid,
        url: &str,
        secret: &str,
        events: &[WebhookEventType],
    ) -> Result<Webhook, Error>;

    async fn get(&self, id: Uuid) -> Result<Option<Webhook>, Error>;

    async fn list_by_repo(&self, repository_id: Uuid) -> Result<Vec<Webhook>, Error>;

    async fn update(
        &self,
        id: Uuid,
        url: Option<&str>,
        secret: Option<&str>,
        events: Option<&[WebhookEventType]>,
    ) -> Result<Webhook, Error>;

    async fn delete(&self, id: Uuid) -> Result<(), Error>;
}

#[derive(Debug, Clone)]
pub struct WebhookRepositoryImpl {
    pool: PgPool,
}

impl WebhookRepositoryImpl {
    pub fn new(pool: PgPool) -> WebhookRepositoryImpl {
        WebhookRepositoryImpl { pool }
    }
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl WebhookRepository for WebhookRepositoryImpl {
    async fn create(
        &self,
        repository_id: Uuid,
        url: &str,
        secret: &str,
        events: &[WebhookEventType],
    ) -> Result<Webhook, Error> {
        let webhook = sqlx::query_as::<_, Webhook>(
            r#"
            INSERT INTO core.webhooks (repository_id, url, secret, events)
            VALUES ($1, $2, $3, $4)
            RETURNING id, repository_id, url, secret, events, created_at, updated_at
            "#,
        )
        .bind(repository_id)
        .bind(url)
        .bind(secret)
        .bind(events)
        .fetch_one(&self.pool)
        .await?;

        Ok(webhook)
    }

    async fn get(&self, id: Uuid) -> Result<Option<Webhook>, Error> {
        let webhook = sqlx::query_as::<_, Webhook>(
            r#"
            SELECT id, repository_id, url, secret, events, created_at, updated_at
            FROM core.webhooks WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(webhook)
    }

    async fn list_by_repo(&self, repository_id: Uuid) -> Result<Vec<Webhook>, Error> {
        let webhooks = sqlx::query_as::<_, Webhook>(
            r#"
            SELECT id, repository_id, url, secret, events, created_at, updated_at
            FROM core.webhooks WHERE repository_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(repository_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(webhooks)
    }

    async fn update(
        &self,
        id: Uuid,
        url: Option<&str>,
        secret: Option<&str>,
        events: Option<&[WebhookEventType]>,
    ) -> Result<Webhook, Error> {
        let webhook = sqlx::query_as::<_, Webhook>(
            r#"
            UPDATE core.webhooks
            SET url = COALESCE($2, url),
                secret = COALESCE($3, secret),
                events = COALESCE($4, events),
                updated_at = now()
            WHERE id = $1
            RETURNING id, repository_id, url, secret, events, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(url)
        .bind(secret)
        .bind(events)
        .fetch_one(&self.pool)
        .await?;

        Ok(webhook)
    }

    async fn delete(&self, id: Uuid) -> Result<(), Error> {
        sqlx::query("DELETE FROM core.webhooks WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
