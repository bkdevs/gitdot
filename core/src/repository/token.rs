use async_trait::async_trait;
use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::model::AccessToken;

#[async_trait]
pub trait TokenRepository: Send + Sync + Clone + 'static {
    async fn create_access_token(
        &self,
        user_id: Uuid,
        client_id: &str,
        token_hash: &str,
    ) -> Result<AccessToken, Error>;

    async fn create_runner_token(
        &self,
        runner_id: Uuid,
        token_hash: &str,
    ) -> Result<AccessToken, Error>;

    async fn get_access_token_by_hash(
        &self,
        token_hash: &str,
    ) -> Result<Option<AccessToken>, Error>;

    async fn touch_access_token(&self, id: Uuid) -> Result<(), Error>;

    async fn delete_access_token(&self, id: Uuid) -> Result<(), Error>;
}

#[derive(Debug, Clone)]
pub struct TokenRepositoryImpl {
    pool: PgPool,
}

impl TokenRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TokenRepository for TokenRepositoryImpl {
    async fn create_access_token(
        &self,
        user_id: Uuid,
        client_id: &str,
        token_hash: &str,
    ) -> Result<AccessToken, Error> {
        let token = sqlx::query_as::<_, AccessToken>(
            r#"
            INSERT INTO tokens (principal_id, client_id, token_hash, token_type)
            VALUES ($1, $2, $3, 'personal')
            RETURNING id, principal_id, client_id, token_hash, token_type, created_at, last_used_at
            "#,
        )
        .bind(user_id)
        .bind(client_id)
        .bind(token_hash)
        .fetch_one(&self.pool)
        .await?;

        Ok(token)
    }

    async fn create_runner_token(
        &self,
        runner_id: Uuid,
        token_hash: &str,
    ) -> Result<AccessToken, Error> {
        let token = sqlx::query_as::<_, AccessToken>(
            r#"
            INSERT INTO tokens (principal_id, client_id, token_hash, token_type)
            VALUES ($1, '', $2, 'runner')
            RETURNING id, principal_id, client_id, token_hash, token_type, created_at, last_used_at
            "#,
        )
        .bind(runner_id)
        .bind(token_hash)
        .fetch_one(&self.pool)
        .await?;

        Ok(token)
    }

    async fn get_access_token_by_hash(
        &self,
        token_hash: &str,
    ) -> Result<Option<AccessToken>, Error> {
        let token = sqlx::query_as::<_, AccessToken>(
            r#"
            SELECT id, principal_id, client_id, token_hash, token_type, created_at, last_used_at
            FROM tokens
            WHERE token_hash = $1
            "#,
        )
        .bind(token_hash)
        .fetch_optional(&self.pool)
        .await?;

        Ok(token)
    }

    async fn touch_access_token(&self, id: Uuid) -> Result<(), Error> {
        sqlx::query("UPDATE tokens SET last_used_at = NOW() WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn delete_access_token(&self, id: Uuid) -> Result<(), Error> {
        sqlx::query("DELETE FROM tokens WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
