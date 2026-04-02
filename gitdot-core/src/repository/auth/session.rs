use async_trait::async_trait;
use chrono::{DateTime, Utc};
use ipnetwork::IpNetwork;
use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::model::{AuthCode, Session};

#[async_trait]
pub trait SessionRepository: Send + Sync + Clone + 'static {
    async fn create_auth_code(
        &self,
        user_id: Uuid,
        code_hash: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<AuthCode, Error>;

    async fn get_auth_code_by_hash(&self, code_hash: &str) -> Result<Option<AuthCode>, Error>;

    async fn mark_auth_code_used(&self, id: Uuid) -> Result<(), Error>;

    async fn create_session(
        &self,
        user_id: Uuid,
        refresh_token_hash: &str,
        refresh_token_family: Uuid,
        user_agent: Option<&str>,
        ip_address: Option<IpNetwork>,
        expires_at: DateTime<Utc>,
    ) -> Result<Session, Error>;

    async fn get_session_by_refresh_hash(&self, hash: &str) -> Result<Option<Session>, Error>;

    async fn revoke_session(&self, id: Uuid) -> Result<(), Error>;

    async fn revoke_sessions_by_family(&self, family: Uuid) -> Result<(), Error>;
}

#[derive(Debug, Clone)]
pub struct SessionRepositoryImpl {
    pool: PgPool,
}

impl SessionRepositoryImpl {
    pub fn new(pool: PgPool) -> SessionRepositoryImpl {
        SessionRepositoryImpl { pool }
    }
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl SessionRepository for SessionRepositoryImpl {
    async fn create_auth_code(
        &self,
        user_id: Uuid,
        code_hash: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<AuthCode, Error> {
        let auth_code = sqlx::query_as::<_, AuthCode>(
            r#"
            INSERT INTO auth.auth_codes (user_id, code_hash, expires_at)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
        )
        .bind(user_id)
        .bind(code_hash)
        .bind(expires_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(auth_code)
    }

    async fn get_auth_code_by_hash(&self, code_hash: &str) -> Result<Option<AuthCode>, Error> {
        let auth_code = sqlx::query_as::<_, AuthCode>(
            r#"
            SELECT * FROM auth.auth_codes WHERE code_hash = $1
            "#,
        )
        .bind(code_hash)
        .fetch_optional(&self.pool)
        .await?;

        Ok(auth_code)
    }

    async fn mark_auth_code_used(&self, id: Uuid) -> Result<(), Error> {
        sqlx::query(
            r#"
            UPDATE auth.auth_codes SET used_at = NOW() WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn create_session(
        &self,
        user_id: Uuid,
        refresh_token_hash: &str,
        refresh_token_family: Uuid,
        user_agent: Option<&str>,
        ip_address: Option<IpNetwork>,
        expires_at: DateTime<Utc>,
    ) -> Result<Session, Error> {
        let session = sqlx::query_as::<_, Session>(
            r#"
            INSERT INTO auth.sessions (user_id, refresh_token_hash, refresh_token_family, user_agent, ip_address, expires_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
        )
        .bind(user_id)
        .bind(refresh_token_hash)
        .bind(refresh_token_family)
        .bind(user_agent)
        .bind(ip_address)
        .bind(expires_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(session)
    }

    async fn get_session_by_refresh_hash(&self, hash: &str) -> Result<Option<Session>, Error> {
        let session = sqlx::query_as::<_, Session>(
            r#"
            SELECT * FROM auth.sessions WHERE refresh_token_hash = $1
            "#,
        )
        .bind(hash)
        .fetch_optional(&self.pool)
        .await?;

        Ok(session)
    }

    async fn revoke_session(&self, id: Uuid) -> Result<(), Error> {
        sqlx::query(
            r#"
            UPDATE auth.sessions SET revoked_at = NOW() WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn revoke_sessions_by_family(&self, family: Uuid) -> Result<(), Error> {
        sqlx::query(
            r#"
            UPDATE auth.sessions SET revoked_at = NOW()
            WHERE refresh_token_family = $1 AND revoked_at IS NULL
            "#,
        )
        .bind(family)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
