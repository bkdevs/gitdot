use async_trait::async_trait;
use chrono::{DateTime, Utc};
use ipnetwork::IpNetwork;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::DatabaseError,
    model::{AuthCode, Session},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthCodeVerification {
    Success,
    Invalid,
    AttemptsExhausted,
    NoActiveCode,
}

/// sqlx data-access layer for the `auth.auth_codes` and `auth.sessions` tables,
/// which back short-lived auth codes and long-lived refresh-token sessions.
#[async_trait]
pub trait SessionRepository: Send + Sync + Clone + 'static {
    /// Inserts an auth code (`user_id`, hashed code, expiry) into
    /// `auth.auth_codes` and returns the created row.
    async fn create_auth_code(
        &self,
        user_id: Uuid,
        code_hash: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<AuthCode, DatabaseError>;

    /// Returns the auth code matching `user_id` and `code_hash`, or `Ok(None)`
    /// if none exists. Does not check expiry or `used_at`.
    async fn get_auth_code(
        &self,
        user_id: Uuid,
        code_hash: &str,
    ) -> Result<Option<AuthCode>, DatabaseError>;

    /// Sets `used_at = NOW()` on the auth code with the given id. No-op (and
    /// still `Ok`) if no row matches.
    async fn mark_auth_code_used(&self, id: Uuid) -> Result<(), DatabaseError>;

    /// Marks every active (unused) auth code for `user_id` as used, so a freshly
    /// issued code is the only one that can verify. Called before inserting a
    /// new code on (re)send to keep exactly one active code per user.
    async fn invalidate_auth_codes(&self, user_id: Uuid) -> Result<(), DatabaseError>;

    /// Atomically verifies a login code against the user's single active code
    /// and consumes or penalizes it in one transaction.
    ///
    /// - correct code → marked used, returns [`AuthCodeVerification::Success`];
    /// - wrong code with budget left → `attempt_count` incremented, returns
    ///   [`AuthCodeVerification::Invalid`];
    /// - wrong code that reaches `max_attempts` → code burned (`used_at =
    ///   NOW()`), returns [`AuthCodeVerification::AttemptsExhausted`];
    /// - no active/unexpired code → [`AuthCodeVerification::NoActiveCode`].
    async fn verify_and_consume_auth_code(
        &self,
        user_id: Uuid,
        code_hash: &str,
        max_attempts: i16,
    ) -> Result<AuthCodeVerification, DatabaseError>;

    /// Inserts a session into `auth.sessions` (`user_id`, hashed refresh token,
    /// `refresh_token_family`, optional user agent and IP, expiry) and returns
    /// the created row.
    async fn create_session(
        &self,
        user_id: Uuid,
        refresh_token_hash: &str,
        refresh_token_family: Uuid,
        user_agent: Option<&str>,
        ip_address: Option<IpNetwork>,
        expires_at: DateTime<Utc>,
    ) -> Result<Session, DatabaseError>;

    /// Returns the session matching `refresh_token_hash`, or `Ok(None)` if none
    /// exists. Does not check expiry or `revoked_at`.
    async fn get_session_by_refresh_hash(
        &self,
        hash: &str,
    ) -> Result<Option<Session>, DatabaseError>;

    /// Revokes a single session by setting `revoked_at = NOW()` on the row with
    /// the given id (soft delete). No-op (and still `Ok`) if no row matches.
    async fn revoke_session(&self, id: Uuid) -> Result<(), DatabaseError>;

    /// Revokes every active session in a refresh-token family by setting
    /// `revoked_at = NOW()` where `refresh_token_family` matches and `revoked_at
    /// IS NULL` (soft delete; already-revoked rows are left untouched).
    async fn revoke_sessions_by_family(&self, family: Uuid) -> Result<(), DatabaseError>;

    /// Revokes every active session for `user_id` by setting `revoked_at =
    /// NOW()` where `revoked_at IS NULL` (soft delete; already-revoked rows are
    /// left untouched). Used when an account is deleted so existing refresh
    /// tokens stop rotating.
    async fn revoke_sessions_by_user(&self, user_id: Uuid) -> Result<(), DatabaseError>;
}

#[derive(Debug, Clone)]
pub struct PgSessionRepository {
    pool: PgPool,
}

impl PgSessionRepository {
    pub fn new(pool: PgPool) -> PgSessionRepository {
        PgSessionRepository { pool }
    }
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl SessionRepository for PgSessionRepository {
    async fn create_auth_code(
        &self,
        user_id: Uuid,
        code_hash: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<AuthCode, DatabaseError> {
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

    async fn get_auth_code(
        &self,
        user_id: Uuid,
        code_hash: &str,
    ) -> Result<Option<AuthCode>, DatabaseError> {
        let auth_code = sqlx::query_as::<_, AuthCode>(
            r#"
            SELECT * FROM auth.auth_codes WHERE user_id = $1 AND code_hash = $2
            "#,
        )
        .bind(user_id)
        .bind(code_hash)
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

    async fn invalidate_auth_codes(&self, user_id: Uuid) -> Result<(), DatabaseError> {
        sqlx::query(
            r#"
            UPDATE auth.auth_codes SET used_at = NOW()
            WHERE user_id = $1 AND used_at IS NULL
            "#,
        )
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn verify_and_consume_auth_code(
        &self,
        user_id: Uuid,
        code_hash: &str,
        max_attempts: i16,
    ) -> Result<AuthCodeVerification, DatabaseError> {
        let mut tx = self.pool.begin().await?;

        // lock the user's single active to prevent concurrent verification
        let code = sqlx::query_as::<_, AuthCode>(
            r#"
            SELECT * FROM auth.auth_codes
            WHERE user_id = $1 AND used_at IS NULL AND expires_at > NOW()
            ORDER BY created_at DESC
            LIMIT 1
            FOR UPDATE
            "#,
        )
        .bind(user_id)
        .fetch_optional(&mut *tx)
        .await?;

        let Some(code) = code else {
            tx.commit().await?;
            return Ok(AuthCodeVerification::NoActiveCode);
        };

        if code.code_hash == code_hash {
            sqlx::query("UPDATE auth.auth_codes SET used_at = NOW() WHERE id = $1")
                .bind(code.id)
                .execute(&mut *tx)
                .await?;
            tx.commit().await?;
            return Ok(AuthCodeVerification::Success);
        }

        let next = code.attempt_count + 1;
        if next >= max_attempts {
            sqlx::query(
                "UPDATE auth.auth_codes SET attempt_count = $2, used_at = NOW() WHERE id = $1",
            )
            .bind(code.id)
            .bind(next)
            .execute(&mut *tx)
            .await?;
            tx.commit().await?;
            return Ok(AuthCodeVerification::AttemptsExhausted);
        }

        sqlx::query("UPDATE auth.auth_codes SET attempt_count = $2 WHERE id = $1")
            .bind(code.id)
            .bind(next)
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;

        Ok(AuthCodeVerification::Invalid)
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

    async fn revoke_sessions_by_user(&self, user_id: Uuid) -> Result<(), DatabaseError> {
        sqlx::query(
            r#"
            UPDATE auth.sessions SET revoked_at = NOW()
            WHERE user_id = $1 AND revoked_at IS NULL
            "#,
        )
        .bind(user_id)
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

    use super::{AuthCodeVerification, PgSessionRepository, SessionRepository};
    use crate::repository::test_common::insert_user;

    #[sqlx::test]
    async fn auth_code_create_get_and_mark_used(pool: PgPool) {
        let repo = PgSessionRepository::new(pool.clone());
        let user = Uuid::new_v4();
        insert_user(&pool, user, "alice").await;

        let code = repo
            .create_auth_code(user, "code-hash", Utc::now() + Duration::hours(1))
            .await
            .unwrap();
        assert_eq!(code.user_id, user);
        assert!(code.used_at.is_none());

        let found = repo
            .get_auth_code(user, "code-hash")
            .await
            .unwrap()
            .expect("found");
        assert_eq!(found.id, code.id);

        repo.mark_auth_code_used(code.id).await.unwrap();
        assert!(
            repo.get_auth_code(user, "code-hash")
                .await
                .unwrap()
                .unwrap()
                .used_at
                .is_some()
        );

        assert!(
            repo.get_auth_code(user, "missing-hash")
                .await
                .unwrap()
                .is_none()
        );
        assert!(
            repo.get_auth_code(Uuid::new_v4(), "code-hash")
                .await
                .unwrap()
                .is_none()
        );
    }

    #[sqlx::test]
    async fn invalidate_auth_codes_marks_active_used(pool: PgPool) {
        let repo = PgSessionRepository::new(pool.clone());
        let user = Uuid::new_v4();
        let other_user = Uuid::new_v4();
        insert_user(&pool, user, "alice").await;
        insert_user(&pool, other_user, "bob").await;
        let exp = Utc::now() + Duration::hours(1);

        repo.create_auth_code(user, "active-1", exp).await.unwrap();
        repo.create_auth_code(user, "active-2", exp).await.unwrap();
        let already_used = repo.create_auth_code(user, "used", exp).await.unwrap();
        repo.mark_auth_code_used(already_used.id).await.unwrap();
        let used_at = repo
            .get_auth_code(user, "used")
            .await
            .unwrap()
            .unwrap()
            .used_at
            .expect("already used");
        // A code belonging to a different user must not be touched.
        repo.create_auth_code(other_user, "other", exp)
            .await
            .unwrap();

        repo.invalidate_auth_codes(user).await.unwrap();

        // Both active codes are now marked used.
        assert!(
            repo.get_auth_code(user, "active-1")
                .await
                .unwrap()
                .unwrap()
                .used_at
                .is_some()
        );
        assert!(
            repo.get_auth_code(user, "active-2")
                .await
                .unwrap()
                .unwrap()
                .used_at
                .is_some()
        );
        // An already-used code keeps its original timestamp (untouched).
        assert_eq!(
            repo.get_auth_code(user, "used")
                .await
                .unwrap()
                .unwrap()
                .used_at,
            Some(used_at)
        );
        // Another user's code stays active.
        assert!(
            repo.get_auth_code(other_user, "other")
                .await
                .unwrap()
                .unwrap()
                .used_at
                .is_none()
        );
    }

    #[sqlx::test]
    async fn verify_consumes_correct_code_once(pool: PgPool) {
        let repo = PgSessionRepository::new(pool.clone());
        let user = Uuid::new_v4();
        insert_user(&pool, user, "alice").await;
        repo.create_auth_code(user, "good", Utc::now() + Duration::hours(1))
            .await
            .unwrap();

        // Correct code succeeds, then can't be reused.
        assert_eq!(
            repo.verify_and_consume_auth_code(user, "good", 5)
                .await
                .unwrap(),
            AuthCodeVerification::Success
        );
        assert_eq!(
            repo.verify_and_consume_auth_code(user, "good", 5)
                .await
                .unwrap(),
            AuthCodeVerification::NoActiveCode
        );
    }

    #[sqlx::test]
    async fn verify_locks_out_after_max_attempts(pool: PgPool) {
        let repo = PgSessionRepository::new(pool.clone());
        let user = Uuid::new_v4();
        insert_user(&pool, user, "alice").await;
        repo.create_auth_code(user, "good", Utc::now() + Duration::hours(1))
            .await
            .unwrap();

        // Four wrong guesses are rejected but keep the code alive.
        for _ in 0..4 {
            assert_eq!(
                repo.verify_and_consume_auth_code(user, "wrong", 5)
                    .await
                    .unwrap(),
                AuthCodeVerification::Invalid
            );
        }
        // The fifth wrong guess exhausts the budget and burns the code...
        assert_eq!(
            repo.verify_and_consume_auth_code(user, "wrong", 5)
                .await
                .unwrap(),
            AuthCodeVerification::AttemptsExhausted
        );
        // ...so even the correct code no longer works.
        assert_eq!(
            repo.verify_and_consume_auth_code(user, "good", 5)
                .await
                .unwrap(),
            AuthCodeVerification::NoActiveCode
        );
    }

    #[sqlx::test]
    async fn verify_rejects_expired_code(pool: PgPool) {
        let repo = PgSessionRepository::new(pool.clone());
        let user = Uuid::new_v4();
        insert_user(&pool, user, "alice").await;
        repo.create_auth_code(user, "good", Utc::now() - Duration::minutes(1))
            .await
            .unwrap();

        assert_eq!(
            repo.verify_and_consume_auth_code(user, "good", 5)
                .await
                .unwrap(),
            AuthCodeVerification::NoActiveCode
        );
    }

    #[sqlx::test]
    async fn invalidate_auth_codes_drops_prior_codes(pool: PgPool) {
        let repo = PgSessionRepository::new(pool.clone());
        let user = Uuid::new_v4();
        insert_user(&pool, user, "alice").await;
        let exp = Utc::now() + Duration::hours(1);

        // Mimic a resend: old code invalidated, new code issued.
        repo.create_auth_code(user, "old", exp).await.unwrap();
        repo.invalidate_auth_codes(user).await.unwrap();
        repo.create_auth_code(user, "new", exp).await.unwrap();

        // Only the newest code verifies.
        assert_eq!(
            repo.verify_and_consume_auth_code(user, "old", 5)
                .await
                .unwrap(),
            AuthCodeVerification::Invalid
        );
        assert_eq!(
            repo.verify_and_consume_auth_code(user, "new", 5)
                .await
                .unwrap(),
            AuthCodeVerification::Success
        );
    }

    #[sqlx::test]
    async fn session_create_get_and_revoke(pool: PgPool) {
        let repo = PgSessionRepository::new(pool.clone());
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
        let repo = PgSessionRepository::new(pool.clone());
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

    #[sqlx::test]
    async fn revoke_sessions_by_user_revokes_only_that_users_active(pool: PgPool) {
        let repo = PgSessionRepository::new(pool.clone());
        let alice = Uuid::new_v4();
        let bob = Uuid::new_v4();
        insert_user(&pool, alice, "alice").await;
        insert_user(&pool, bob, "bob").await;
        let exp = Utc::now() + Duration::days(7);

        // Alice has two sessions across different families; bob has one.
        repo.create_session(alice, "a1", Uuid::new_v4(), None, None, exp)
            .await
            .unwrap();
        repo.create_session(alice, "a2", Uuid::new_v4(), None, None, exp)
            .await
            .unwrap();
        repo.create_session(bob, "b1", Uuid::new_v4(), None, None, exp)
            .await
            .unwrap();

        repo.revoke_sessions_by_user(alice).await.unwrap();

        // All of alice's sessions are revoked, regardless of family.
        assert!(
            repo.get_session_by_refresh_hash("a1")
                .await
                .unwrap()
                .unwrap()
                .revoked_at
                .is_some()
        );
        assert!(
            repo.get_session_by_refresh_hash("a2")
                .await
                .unwrap()
                .unwrap()
                .revoked_at
                .is_some()
        );
        // Bob's session is untouched.
        assert!(
            repo.get_session_by_refresh_hash("b1")
                .await
                .unwrap()
                .unwrap()
                .revoked_at
                .is_none()
        );
    }
}
