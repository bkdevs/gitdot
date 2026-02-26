use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::model::DeviceAuthorization;

#[async_trait]
pub trait CodeRepository: Send + Sync + Clone + 'static {
    async fn create_device_authorization(
        &self,
        device_code: &str,
        user_code: &str,
        client_id: &str,
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

    async fn expire_device_authorization(&self, id: Uuid) -> Result<(), Error>;

    async fn authorize_device(
        &self,
        user_code: &str,
        user_id: Uuid,
    ) -> Result<Option<DeviceAuthorization>, Error>;

    async fn deny_device(
        &self,
        user_code: &str,
        user_id: Uuid,
    ) -> Result<Option<DeviceAuthorization>, Error>;
}

#[derive(Debug, Clone)]
pub struct CodeRepositoryImpl {
    pool: PgPool,
}

impl CodeRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CodeRepository for CodeRepositoryImpl {
    async fn create_device_authorization(
        &self,
        device_code: &str,
        user_code: &str,
        client_id: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<DeviceAuthorization, Error> {
        let device_auth = sqlx::query_as::<_, DeviceAuthorization>(
            r#"
            INSERT INTO device_authorizations (device_code, user_code, client_id, expires_at)
            VALUES ($1, $2, $3, $4)
            RETURNING id, device_code, user_code, client_id, user_id, status, expires_at, created_at
            "#,
        )
        .bind(device_code)
        .bind(user_code)
        .bind(client_id)
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
            SELECT id, device_code, user_code, client_id, user_id, status, expires_at, created_at
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
            SELECT id, device_code, user_code, client_id, user_id, status, expires_at, created_at
            FROM device_authorizations
            WHERE user_code = $1
            "#,
        )
        .bind(user_code)
        .fetch_optional(&self.pool)
        .await?;

        Ok(device_auth)
    }

    async fn expire_device_authorization(&self, id: Uuid) -> Result<(), Error> {
        sqlx::query(
            r#"
            UPDATE device_authorizations
            SET status = 'expired'
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
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
            RETURNING id, device_code, user_code, client_id, user_id, status, expires_at, created_at
            "#,
        )
        .bind(user_code)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(device_auth)
    }

    async fn deny_device(
        &self,
        user_code: &str,
        user_id: Uuid,
    ) -> Result<Option<DeviceAuthorization>, Error> {
        let device_auth = sqlx::query_as::<_, DeviceAuthorization>(
            r#"
            UPDATE device_authorizations
            SET status = 'denied', user_id = $2
            WHERE user_code = $1 AND status = 'pending' AND expires_at > NOW()
            RETURNING id, device_code, user_code, client_id, user_id, status, expires_at, created_at
            "#,
        )
        .bind(user_code)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(device_auth)
    }
}
