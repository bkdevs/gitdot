use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::model::{AccessToken, DeviceAuthorization};

#[async_trait]
pub trait OAuthRepository: Send + Sync + Clone + 'static {
    async fn create_device_authorization(
        &self,
        device_code: &str,
        user_code: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<DeviceAuthorization, Error>;

    async fn get_device_authorization_by_device_code(
        &self,
        device_code: &str,
    ) -> Result<Option<DeviceAuthorization>, Error>;

    async fn get_device_authorization_by_user_code(
        &self,
        user_code: &str,
    ) -> Result<Option<DeviceAuthorization>, Error>;

    async fn authorize_device(
        &self,
        user_code: &str,
        user_id: Uuid,
    ) -> Result<Option<DeviceAuthorization>, Error>;

    async fn deny_device(&self, user_code: &str) -> Result<Option<DeviceAuthorization>, Error>;

    async fn create_access_token(
        &self,
        user_id: Uuid,
        token_hash: &str,
        name: Option<&str>,
    ) -> Result<AccessToken, Error>;

    async fn get_access_token_by_hash(
        &self,
        token_hash: &str,
    ) -> Result<Option<AccessToken>, Error>;

    async fn touch_access_token(&self, id: Uuid) -> Result<(), Error>;

    async fn delete_access_token(&self, id: Uuid) -> Result<(), Error>;
}

#[derive(Debug, Clone)]
pub struct OAuthRepositoryImpl {
    pool: PgPool,
}

impl OAuthRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl OAuthRepository for OAuthRepositoryImpl {
    async fn create_device_authorization(
        &self,
        device_code: &str,
        user_code: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<DeviceAuthorization, Error> {
        let device_auth = sqlx::query_as::<_, DeviceAuthorization>(
            r#"
            INSERT INTO device_authorizations (device_code, user_code, expires_at)
            VALUES ($1, $2, $3)
            RETURNING id, device_code, user_code, user_id, status, expires_at, created_at
            "#,
        )
        .bind(device_code)
        .bind(user_code)
        .bind(expires_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(device_auth)
    }

    async fn get_device_authorization_by_device_code(
        &self,
        device_code: &str,
    ) -> Result<Option<DeviceAuthorization>, Error> {
        let device_auth = sqlx::query_as::<_, DeviceAuthorization>(
            r#"
            SELECT id, device_code, user_code, user_id, status, expires_at, created_at
            FROM device_authorizations
            WHERE device_code = $1
            "#,
        )
        .bind(device_code)
        .fetch_optional(&self.pool)
        .await?;

        Ok(device_auth)
    }

    async fn get_device_authorization_by_user_code(
        &self,
        user_code: &str,
    ) -> Result<Option<DeviceAuthorization>, Error> {
        let device_auth = sqlx::query_as::<_, DeviceAuthorization>(
            r#"
            SELECT id, device_code, user_code, user_id, status, expires_at, created_at
            FROM device_authorizations
            WHERE user_code = $1
            "#,
        )
        .bind(user_code)
        .fetch_optional(&self.pool)
        .await?;

        Ok(device_auth)
    }

    async fn authorize_device(
        &self,
        user_code: &str,
        user_id: Uuid,
    ) -> Result<Option<DeviceAuthorization>, Error> {
        let device_auth = sqlx::query_as::<_, DeviceAuthorization>(
            r#"
            UPDATE device_authorizations
            SET status = 'authorized', user_id = $2
            WHERE user_code = $1 AND status = 'pending' AND expires_at > NOW()
            RETURNING id, device_code, user_code, user_id, status, expires_at, created_at
            "#,
        )
        .bind(user_code)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(device_auth)
    }

    async fn deny_device(&self, user_code: &str) -> Result<Option<DeviceAuthorization>, Error> {
        let device_auth = sqlx::query_as::<_, DeviceAuthorization>(
            r#"
            UPDATE device_authorizations
            SET status = 'denied'
            WHERE user_code = $1 AND status = 'pending'
            RETURNING id, device_code, user_code, user_id, status, expires_at, created_at
            "#,
        )
        .bind(user_code)
        .fetch_optional(&self.pool)
        .await?;

        Ok(device_auth)
    }

    async fn create_access_token(
        &self,
        user_id: Uuid,
        token_hash: &str,
        name: Option<&str>,
    ) -> Result<AccessToken, Error> {
        let token = sqlx::query_as::<_, AccessToken>(
            r#"
            INSERT INTO access_tokens (user_id, token_hash, name)
            VALUES ($1, $2, $3)
            RETURNING id, user_id, token_hash, name, last_used_at, created_at
            "#,
        )
        .bind(user_id)
        .bind(token_hash)
        .bind(name)
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
            SELECT id, user_id, token_hash, name, last_used_at, created_at
            FROM access_tokens
            WHERE token_hash = $1
            "#,
        )
        .bind(token_hash)
        .fetch_optional(&self.pool)
        .await?;

        Ok(token)
    }

    async fn touch_access_token(&self, id: Uuid) -> Result<(), Error> {
        sqlx::query("UPDATE access_tokens SET last_used_at = NOW() WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn delete_access_token(&self, id: Uuid) -> Result<(), Error> {
        sqlx::query("DELETE FROM access_tokens WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
