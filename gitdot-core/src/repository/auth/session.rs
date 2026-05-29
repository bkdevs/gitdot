use async_trait::async_trait;
use chrono::{DateTime, Utc};
use ipnetwork::IpNetwork;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::DatabaseError,
    model::{AuthCode, Session},
};

#[async_trait]
pub trait SessionRepository: Send + Sync + Clone + 'static {
    async fn create_auth_code(
        &self,
        user_id: Uuid,
        user_code: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<AuthCode, DatabaseError>;

    async fn get_auth_code(&self, user_code: &str) -> Result<Option<AuthCode>, DatabaseError>;

    async fn mark_auth_code_used(&self, id: Uuid) -> Result<(), DatabaseError>;

    async fn create_session(
        &self,
        user_id: Uuid,
        refresh_token_hash: &str,
        refresh_token_family: Uuid,
        user_agent: Option<&str>,
        ip_address: Option<IpNetwork>,
        expires_at: DateTime<Utc>,
    ) -> Result<Session, DatabaseError>;

    async fn get_session_by_refresh_hash(
        &self,
        hash: &str,
    ) -> Result<Option<Session>, DatabaseError>;

    async fn revoke_session(&self, id: Uuid) -> Result<(), DatabaseError>;

    async fn revoke_sessions_by_family(&self, family: Uuid) -> Result<(), DatabaseError>;
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
        user_code: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<AuthCode, DatabaseError> {
        let auth_code = sqlx::query_as::<_, AuthCode>(
            r#"
            INSERT INTO auth.auth_codes (user_id, user_code, expires_at)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
        )
        .bind(user_id)
        .bind(user_code)
        .bind(expires_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(auth_code)
    }

    async fn get_auth_code(&self, user_code: &str) -> Result<Option<AuthCode>, DatabaseError> {
        let auth_code = sqlx::query_as::<_, AuthCode>(
            r#"
            SELECT * FROM auth.auth_codes WHERE user_code = $1
            "#,
        )
        .bind(user_code)
        .fetch_optional(&self.pool)
        .await?;

        Ok(auth_code)
    }

    async fn mark_auth_code_used(&self, id: Uuid) -> Result<(), DatabaseError> {
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
    ) -> Result<Session, DatabaseError> {
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

    async fn get_session_by_refresh_hash(
        &self,
        hash: &str,
    ) -> Result<Option<Session>, DatabaseError> {
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

    async fn revoke_session(&self, id: Uuid) -> Result<(), DatabaseError> {
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

    async fn revoke_sessions_by_family(&self, family: Uuid) -> Result<(), DatabaseError> {
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

#[cfg(all(test, feature = "db-tests"))]
mod tests {
    use chrono::{Duration, Utc};
    use ipnetwork::IpNetwork;
    use sqlx::PgPool;
    use uuid::Uuid;

    use super::{SessionRepository, SessionRepositoryImpl};
    use crate::repository::test_common::insert_user;

    #[sqlx::test]
    async fn auth_code_create_get_and_mark_used(pool: PgPool) {
        let repo = SessionRepositoryImpl::new(pool.clone());
        let user = Uuid::new_v4();
        insert_user(&pool, user, "alice").await;

        let code = repo
            .create_auth_code(user, "USER-CODE", Utc::now() + Duration::hours(1))
            .await
            .unwrap();
        assert_eq!(code.user_id, user);
        assert!(code.used_at.is_none());

        let found = repo
            .get_auth_code("USER-CODE")
            .await
            .unwrap()
            .expect("found");
        assert_eq!(found.id, code.id);

        repo.mark_auth_code_used(code.id).await.unwrap();
        assert!(
            repo.get_auth_code("USER-CODE")
                .await
                .unwrap()
                .unwrap()
                .used_at
                .is_some()
        );

        assert!(repo.get_auth_code("MISSING").await.unwrap().is_none());
    }

    #[sqlx::test]
    async fn session_create_get_and_revoke(pool: PgPool) {
        let repo = SessionRepositoryImpl::new(pool.clone());
        let user = Uuid::new_v4();
        insert_user(&pool, user, "alice").await;
        let family = Uuid::new_v4();
        let ip: IpNetwork = "192.168.1.1".parse().unwrap();

        let session = repo
            .create_session(
                user,
                "rth",
                family,
                Some("agent/1.0"),
                Some(ip),
                Utc::now() + Duration::days(7),
            )
            .await
            .unwrap();
        assert_eq!(session.user_id, user);
        assert_eq!(session.refresh_token_family, family);
        assert_eq!(session.user_agent.as_deref(), Some("agent/1.0"));
        assert_eq!(session.ip_address, Some(ip));
        assert!(session.revoked_at.is_none());

        let found = repo
            .get_session_by_refresh_hash("rth")
            .await
            .unwrap()
            .expect("found");
        assert_eq!(found.id, session.id);

        repo.revoke_session(session.id).await.unwrap();
        assert!(
            repo.get_session_by_refresh_hash("rth")
                .await
                .unwrap()
                .unwrap()
                .revoked_at
                .is_some()
        );

        assert!(
            repo.get_session_by_refresh_hash("missing")
                .await
                .unwrap()
                .is_none()
        );
    }

    #[sqlx::test]
    async fn revoke_sessions_by_family_revokes_active(pool: PgPool) {
        let repo = SessionRepositoryImpl::new(pool.clone());
        let user = Uuid::new_v4();
        insert_user(&pool, user, "alice").await;
        let family = Uuid::new_v4();
        let other_family = Uuid::new_v4();
        let exp = Utc::now() + Duration::days(7);

        repo.create_session(user, "h1", family, None, None, exp)
            .await
            .unwrap();
        repo.create_session(user, "h2", family, None, None, exp)
            .await
            .unwrap();
        repo.create_session(user, "h3", other_family, None, None, exp)
            .await
            .unwrap();

        repo.revoke_sessions_by_family(family).await.unwrap();

        assert!(
            repo.get_session_by_refresh_hash("h1")
                .await
                .unwrap()
                .unwrap()
                .revoked_at
                .is_some()
        );
        assert!(
            repo.get_session_by_refresh_hash("h2")
                .await
                .unwrap()
                .unwrap()
                .revoked_at
                .is_some()
        );
        // A session from a different family stays active.
        assert!(
            repo.get_session_by_refresh_hash("h3")
                .await
                .unwrap()
                .unwrap()
                .revoked_at
                .is_none()
        );
    }
}
