use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{error::DatabaseError, model::DeviceAuthorization};

/// sqlx data-access layer for the `auth.device_authorizations` table, which
/// backs the OAuth device-authorization flow.
#[async_trait]
pub trait DeviceRepository: Send + Sync + Clone + 'static {
    /// Inserts a new `pending` device authorization with the given hashed
    /// device code, hashed user code, client id, and expiry, and returns the
    /// created row.
    async fn create_device_authorization(
        &self,
        device_code_hash: &str,
        user_code_hash: &str,
        client_id: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<DeviceAuthorization, DatabaseError>;

    /// Returns the device authorization matching `device_code_hash`, or
    /// `Ok(None)` if none exists. Does not check expiry or status.
    async fn get_device_authorization_by_device_code_hash(
        &self,
        device_code_hash: &str,
    ) -> Result<Option<DeviceAuthorization>, DatabaseError>;

    /// Returns the device authorization matching `user_code_hash`, or `Ok(None)`
    /// if none exists. Does not check expiry or status.
    async fn get_device_authorization_by_user_code_hash(
        &self,
        user_code_hash: &str,
    ) -> Result<Option<DeviceAuthorization>, DatabaseError>;

    /// Sets `status = 'expired'` on the row with the given id. No-op (and still
    /// `Ok`) if no row matches.
    async fn expire_device_authorization(&self, id: Uuid) -> Result<(), DatabaseError>;

    /// Marks the matching row `authorized` and sets `user_id`, only if it is
    /// still `pending` and unexpired (`expires_at > NOW()`). Returns the updated
    /// row, or `Ok(None)` if no `pending`/unexpired row matched the user code.
    async fn authorize_device(
        &self,
        user_code_hash: &str,
        user_id: Uuid,
    ) -> Result<Option<DeviceAuthorization>, DatabaseError>;

    /// Marks the matching row `denied` and sets `user_id`, only if it is still
    /// `pending` and unexpired (`expires_at > NOW()`). Returns the updated row,
    /// or `Ok(None)` if no `pending`/unexpired row matched the user code.
    async fn deny_device(
        &self,
        user_code_hash: &str,
        user_id: Uuid,
    ) -> Result<Option<DeviceAuthorization>, DatabaseError>;
}

#[derive(Debug, Clone)]
pub struct PgDeviceRepository {
    pool: PgPool,
}

impl PgDeviceRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl DeviceRepository for PgDeviceRepository {
    async fn create_device_authorization(
        &self,
        device_code_hash: &str,
        user_code_hash: &str,
        client_id: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<DeviceAuthorization, DatabaseError> {
        let device_auth = sqlx::query_as::<_, DeviceAuthorization>(
            r#"
            INSERT INTO auth.device_authorizations (device_code_hash, user_code_hash, client_id, expires_at)
            VALUES ($1, $2, $3, $4)
            RETURNING id, device_code_hash, user_code_hash, client_id, user_id, status, expires_at, created_at
            "#,
        )
        .bind(device_code_hash)
        .bind(user_code_hash)
        .bind(client_id)
        .bind(expires_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(device_auth)
    }

    async fn get_device_authorization_by_device_code_hash(
        &self,
        device_code_hash: &str,
    ) -> Result<Option<DeviceAuthorization>, DatabaseError> {
        let device_auth = sqlx::query_as::<_, DeviceAuthorization>(
            r#"
            SELECT id, device_code_hash, user_code_hash, client_id, user_id, status, expires_at, created_at
            FROM auth.device_authorizations
            WHERE device_code_hash = $1
            "#,
        )
        .bind(device_code_hash)
        .fetch_optional(&self.pool)
        .await?;

        Ok(device_auth)
    }

    async fn get_device_authorization_by_user_code_hash(
        &self,
        user_code_hash: &str,
    ) -> Result<Option<DeviceAuthorization>, DatabaseError> {
        let device_auth = sqlx::query_as::<_, DeviceAuthorization>(
            r#"
            SELECT id, device_code_hash, user_code_hash, client_id, user_id, status, expires_at, created_at
            FROM auth.device_authorizations
            WHERE user_code_hash = $1
            "#,
        )
        .bind(user_code_hash)
        .fetch_optional(&self.pool)
        .await?;

        Ok(device_auth)
    }

    async fn expire_device_authorization(&self, id: Uuid) -> Result<(), DatabaseError> {
        sqlx::query(
            r#"
            UPDATE auth.device_authorizations
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
        user_code_hash: &str,
        user_id: Uuid,
    ) -> Result<Option<DeviceAuthorization>, DatabaseError> {
        let device_auth = sqlx::query_as::<_, DeviceAuthorization>(
            r#"
            UPDATE auth.device_authorizations
            SET status = 'authorized', user_id = $2
            WHERE user_code_hash = $1 AND status = 'pending' AND expires_at > NOW()
            RETURNING id, device_code_hash, user_code_hash, client_id, user_id, status, expires_at, created_at
            "#,
        )
        .bind(user_code_hash)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(device_auth)
    }

    async fn deny_device(
        &self,
        user_code_hash: &str,
        user_id: Uuid,
    ) -> Result<Option<DeviceAuthorization>, DatabaseError> {
        let device_auth = sqlx::query_as::<_, DeviceAuthorization>(
            r#"
            UPDATE auth.device_authorizations
            SET status = 'denied', user_id = $2
            WHERE user_code_hash = $1 AND status = 'pending' AND expires_at > NOW()
            RETURNING id, device_code_hash, user_code_hash, client_id, user_id, status, expires_at, created_at
            "#,
        )
        .bind(user_code_hash)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(device_auth)
    }
}

#[cfg(all(test, feature = "db-tests"))]
mod tests {
    use chrono::{Duration, Utc};
    use sqlx::PgPool;
    use uuid::Uuid;

    use super::{DeviceRepository, PgDeviceRepository};
    use crate::{model::DeviceAuthorizationStatus, repository::test_common::insert_user};

    #[sqlx::test]
    async fn create_and_get_device_authorization(pool: PgPool) {
        let repo = PgDeviceRepository::new(pool.clone());
        let created = repo
            .create_device_authorization("dch", "USER-CODE", "cli", Utc::now() + Duration::hours(1))
            .await
            .unwrap();
        assert_eq!(created.device_code_hash, "dch");
        assert_eq!(created.user_code_hash, "USER-CODE");
        assert_eq!(created.client_id, "cli");
        assert_eq!(created.status, DeviceAuthorizationStatus::Pending);
        assert!(created.user_id.is_none());

        let by_hash = repo
            .get_device_authorization_by_device_code_hash("dch")
            .await
            .unwrap()
            .expect("found by device code hash");
        assert_eq!(by_hash.id, created.id);

        let by_code = repo
            .get_device_authorization_by_user_code_hash("USER-CODE")
            .await
            .unwrap()
            .expect("found by user code");
        assert_eq!(by_code.id, created.id);

        assert!(
            repo.get_device_authorization_by_device_code_hash("nope")
                .await
                .unwrap()
                .is_none()
        );
        assert!(
            repo.get_device_authorization_by_user_code_hash("NOPE")
                .await
                .unwrap()
                .is_none()
        );
    }

    #[sqlx::test]
    async fn expire_device_authorization_sets_status(pool: PgPool) {
        let repo = PgDeviceRepository::new(pool.clone());
        let created = repo
            .create_device_authorization("dch", "CODE", "cli", Utc::now() + Duration::hours(1))
            .await
            .unwrap();

        repo.expire_device_authorization(created.id).await.unwrap();

        let after = repo
            .get_device_authorization_by_user_code_hash("CODE")
            .await
            .unwrap()
            .unwrap();
        assert_eq!(after.status, DeviceAuthorizationStatus::Expired);
    }

    #[sqlx::test]
    async fn authorize_device_requires_pending_and_unexpired(pool: PgPool) {
        let repo = PgDeviceRepository::new(pool.clone());
        let user = Uuid::new_v4();
        insert_user(&pool, user, "alice").await;

        repo.create_device_authorization("dch", "CODE", "cli", Utc::now() + Duration::hours(1))
            .await
            .unwrap();

        let authorized = repo
            .authorize_device("CODE", user)
            .await
            .unwrap()
            .expect("authorized");
        assert_eq!(authorized.status, DeviceAuthorizationStatus::Authorized);
        assert_eq!(authorized.user_id, Some(user));

        // A second authorize finds no pending row.
        assert!(repo.authorize_device("CODE", user).await.unwrap().is_none());

        // An unknown user code matches nothing.
        assert!(
            repo.authorize_device("MISSING", user)
                .await
                .unwrap()
                .is_none()
        );

        // A time-expired pending code cannot be authorized.
        repo.create_device_authorization("dch2", "EXPIRED", "cli", Utc::now() - Duration::hours(1))
            .await
            .unwrap();
        assert!(
            repo.authorize_device("EXPIRED", user)
                .await
                .unwrap()
                .is_none()
        );
    }
}
