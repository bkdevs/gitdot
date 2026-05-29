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

#[cfg(all(test, feature = "db-tests"))]
mod tests {
    use sqlx::PgPool;
    use uuid::Uuid;

    use super::{TokenRepository, TokenRepositoryImpl, TokenType};

    #[sqlx::test]
    async fn create_and_get_token(pool: PgPool) {
        let repo = TokenRepositoryImpl::new(pool.clone());
        let principal = Uuid::new_v4();

        let token = repo
            .create_token(principal, "cli", "hash", TokenType::Personal)
            .await
            .unwrap();
        assert_eq!(token.principal_id, principal);
        assert_eq!(token.client_id, "cli");
        assert_eq!(token.token_hash, "hash");
        assert_eq!(token.token_type, TokenType::Personal);
        assert!(token.last_used_at.is_none());

        let found = repo
            .get_token_by_hash("hash")
            .await
            .unwrap()
            .expect("found");
        assert_eq!(found.id, token.id);
        assert!(repo.get_token_by_hash("missing").await.unwrap().is_none());
    }

    #[sqlx::test]
    async fn touch_token_sets_last_used(pool: PgPool) {
        let repo = TokenRepositoryImpl::new(pool.clone());
        let token = repo
            .create_token(Uuid::new_v4(), "cli", "hash", TokenType::Runner)
            .await
            .unwrap();
        assert_eq!(token.token_type, TokenType::Runner);

        repo.touch_token(token.id).await.unwrap();
        assert!(
            repo.get_token_by_hash("hash")
                .await
                .unwrap()
                .unwrap()
                .last_used_at
                .is_some()
        );
    }

    #[sqlx::test]
    async fn delete_token_removes_it(pool: PgPool) {
        let repo = TokenRepositoryImpl::new(pool.clone());
        let token = repo
            .create_token(Uuid::new_v4(), "cli", "hash", TokenType::Personal)
            .await
            .unwrap();

        repo.delete_token(token.id).await.unwrap();
        assert!(repo.get_token_by_hash("hash").await.unwrap().is_none());
    }

    #[sqlx::test]
    async fn delete_token_by_principal_removes_all_their_tokens(pool: PgPool) {
        let repo = TokenRepositoryImpl::new(pool.clone());
        let principal = Uuid::new_v4();
        let other = Uuid::new_v4();
        repo.create_token(principal, "cli", "h1", TokenType::Personal)
            .await
            .unwrap();
        repo.create_token(principal, "cli", "h2", TokenType::Runner)
            .await
            .unwrap();
        repo.create_token(other, "cli", "h3", TokenType::Personal)
            .await
            .unwrap();

        repo.delete_token_by_principal(principal).await.unwrap();

        assert!(repo.get_token_by_hash("h1").await.unwrap().is_none());
        assert!(repo.get_token_by_hash("h2").await.unwrap().is_none());
        // Another principal's token is untouched.
        assert!(repo.get_token_by_hash("h3").await.unwrap().is_some());
    }
}
