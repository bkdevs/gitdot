use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::DatabaseError,
    model::{AccessToken, TokenType},
};

#[async_trait]
pub trait TokenRepository: Send + Sync + Clone + 'static {
    async fn create_token(
        &self,
        principal_id: Uuid,
        client_id: &str,
        token_hash: &str,
        token_type: TokenType,
    ) -> Result<AccessToken, DatabaseError>;

    async fn get_token_by_hash(
        &self,
        token_hash: &str,
    ) -> Result<Option<AccessToken>, DatabaseError>;

    async fn touch_token(&self, id: Uuid) -> Result<(), DatabaseError>;

    async fn delete_token(&self, id: Uuid) -> Result<(), DatabaseError>;

    async fn delete_token_by_principal(&self, principal_id: Uuid) -> Result<(), DatabaseError>;
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

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl TokenRepository for TokenRepositoryImpl {
    async fn create_token(
        &self,
        principal_id: Uuid,
        client_id: &str,
        token_hash: &str,
        token_type: TokenType,
    ) -> Result<AccessToken, DatabaseError> {
        let token = sqlx::query_as::<_, AccessToken>(
            r#"
            INSERT INTO auth.tokens (principal_id, client_id, token_hash, token_type)
            VALUES ($1, $2, $3, $4)
            RETURNING id, principal_id, client_id, token_hash, token_type, created_at, last_used_at
            "#,
        )
        .bind(principal_id)
        .bind(client_id)
        .bind(token_hash)
        .bind(token_type)
        .fetch_one(&self.pool)
        .await?;

        Ok(token)
    }

    async fn get_token_by_hash(
        &self,
        token_hash: &str,
    ) -> Result<Option<AccessToken>, DatabaseError> {
        let token = sqlx::query_as::<_, AccessToken>(
            r#"
            SELECT id, principal_id, client_id, token_hash, token_type, created_at, last_used_at
            FROM auth.tokens
            WHERE token_hash = $1
            "#,
        )
        .bind(token_hash)
        .fetch_optional(&self.pool)
        .await?;

        Ok(token)
    }

    async fn touch_token(&self, id: Uuid) -> Result<(), DatabaseError> {
        sqlx::query("UPDATE auth.tokens SET last_used_at = NOW() WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn delete_token(&self, id: Uuid) -> Result<(), DatabaseError> {
        sqlx::query("DELETE FROM auth.tokens WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn delete_token_by_principal(&self, principal_id: Uuid) -> Result<(), DatabaseError> {
        sqlx::query("DELETE FROM auth.tokens WHERE principal_id = $1")
            .bind(principal_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
