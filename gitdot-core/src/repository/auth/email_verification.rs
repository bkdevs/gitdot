use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::DatabaseError,
    model::{EmailVerificationCode, UserEmail},
};

#[derive(Debug, Clone)]
pub enum EmailCodeVerification {
    Success(UserEmail),
    Invalid,
    AttemptsExhausted,
    NoActiveCode,
}

/// sqlx data-access layer for the `auth.email_verification_codes` table, with
/// writes against `core.user_emails` when consuming a code.
#[async_trait]
pub trait EmailVerificationRepository: Send + Sync + Clone + 'static {
    /// Inserts a verification code (`user_id`, `email`, hashed code, expiry) and
    /// returns the created row.
    async fn create_code(
        &self,
        user_id: Uuid,
        email: &str,
        code_hash: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<EmailVerificationCode, DatabaseError>;

    /// Returns the verification code matching `code_hash`, or `Ok(None)` if none
    /// exists. Does not check expiry or `used_at`.
    async fn get_code_by_hash(
        &self,
        code_hash: &str,
    ) -> Result<Option<EmailVerificationCode>, DatabaseError>;

    /// Invalidates all outstanding codes for `(user_id, email)` by setting
    /// `used_at = NOW()` on rows where they match and `used_at IS NULL`.
    async fn invalidate_codes_for_email(
        &self,
        user_id: Uuid,
        email: &str,
    ) -> Result<(), DatabaseError>;

    /// Verifies `code_hash` against the latest active (unused, unexpired) code
    /// for `(user_id, email)`, enforcing an attempt budget. In one transaction:
    /// a matching hash marks the code used and creates/flips the verified
    /// `core.user_emails` row, returning [`EmailCodeVerification::Success`]; a
    /// wrong hash increments `attempt_count` and burns the code once it reaches
    /// `max_attempts` ([`EmailCodeVerification::AttemptsExhausted`]). A `23505`
    /// from the email insert (a *different* user already verified the address)
    /// is propagated as a `DatabaseError` for the caller to map to a conflict.
    async fn verify_and_consume_email_code(
        &self,
        user_id: Uuid,
        email: &str,
        code_hash: &str,
        max_attempts: i16,
    ) -> Result<EmailCodeVerification, DatabaseError>;
}

#[derive(Debug, Clone)]
pub struct PgEmailVerificationRepository {
    pool: PgPool,
}

impl PgEmailVerificationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl EmailVerificationRepository for PgEmailVerificationRepository {
    async fn create_code(
        &self,
        user_id: Uuid,
        email: &str,
        code_hash: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<EmailVerificationCode, DatabaseError> {
        let code = sqlx::query_as::<_, EmailVerificationCode>(
            r#"
            INSERT INTO auth.email_verification_codes (user_id, email, code_hash, expires_at)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
        )
        .bind(user_id)
        .bind(email)
        .bind(code_hash)
        .bind(expires_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(code)
    }

    async fn get_code_by_hash(
        &self,
        code_hash: &str,
    ) -> Result<Option<EmailVerificationCode>, DatabaseError> {
        let code = sqlx::query_as::<_, EmailVerificationCode>(
            r#"
            SELECT * FROM auth.email_verification_codes WHERE code_hash = $1
            "#,
        )
        .bind(code_hash)
        .fetch_optional(&self.pool)
        .await?;

        Ok(code)
    }

    async fn invalidate_codes_for_email(
        &self,
        user_id: Uuid,
        email: &str,
    ) -> Result<(), DatabaseError> {
        sqlx::query(
            r#"
            UPDATE auth.email_verification_codes
            SET used_at = NOW()
            WHERE user_id = $1 AND email = $2 AND used_at IS NULL
            "#,
        )
        .bind(user_id)
        .bind(email)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn verify_and_consume_email_code(
        &self,
        user_id: Uuid,
        email: &str,
        code_hash: &str,
        max_attempts: i16,
    ) -> Result<EmailCodeVerification, DatabaseError> {
        let mut tx = self.pool.begin().await?;

        // lock the latest active code for this (user, email) to prevent
        // concurrent verifications
        let code = sqlx::query_as::<_, EmailVerificationCode>(
            r#"
            SELECT * FROM auth.email_verification_codes
            WHERE user_id = $1 AND email = $2 AND used_at IS NULL AND expires_at > NOW()
            ORDER BY created_at DESC
            LIMIT 1
            FOR UPDATE
            "#,
        )
        .bind(user_id)
        .bind(email)
        .fetch_optional(&mut *tx)
        .await?;

        let Some(code) = code else {
            tx.commit().await?;
            return Ok(EmailCodeVerification::NoActiveCode);
        };

        if code.code_hash != code_hash {
            let next = code.attempt_count + 1;
            if next >= max_attempts {
                sqlx::query(
                    "UPDATE auth.email_verification_codes SET attempt_count = $2, used_at = NOW() WHERE id = $1",
                )
                .bind(code.id)
                .bind(next)
                .execute(&mut *tx)
                .await?;
                tx.commit().await?;
                return Ok(EmailCodeVerification::AttemptsExhausted);
            }

            sqlx::query(
                "UPDATE auth.email_verification_codes SET attempt_count = $2 WHERE id = $1",
            )
            .bind(code.id)
            .bind(next)
            .execute(&mut *tx)
            .await?;
            tx.commit().await?;
            return Ok(EmailCodeVerification::Invalid);
        }

        sqlx::query("UPDATE auth.email_verification_codes SET used_at = NOW() WHERE id = $1")
            .bind(code.id)
            .execute(&mut *tx)
            .await?;

        let row = sqlx::query_as::<_, UserEmail>(
            r#"
            INSERT INTO core.user_emails (user_id, email, is_primary, is_verified, verified_at)
            VALUES ($1, $2, FALSE, TRUE, NOW())
            ON CONFLICT (user_id, email)
            DO UPDATE SET is_verified = TRUE,
                          verified_at = COALESCE(core.user_emails.verified_at, NOW())
            RETURNING id, user_id, email, is_primary, is_verified, created_at
            "#,
        )
        .bind(user_id)
        .bind(email)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(EmailCodeVerification::Success(row))
    }
}

#[cfg(all(test, feature = "db-tests"))]
mod tests {
    use chrono::{Duration, Utc};
    use sqlx::PgPool;
    use uuid::Uuid;

    use super::{
        EmailCodeVerification, EmailVerificationRepository, PgEmailVerificationRepository,
    };
    use crate::repository::test_common::insert_user;

    const MAX: i16 = 5;

    #[sqlx::test]
    async fn create_and_get_code(pool: PgPool) {
        let repo = PgEmailVerificationRepository::new(pool.clone());
        let user = Uuid::new_v4();
        insert_user(&pool, user, "alice").await;

        let code = repo
            .create_code(user, "alice@x.com", "hash", Utc::now() + Duration::hours(1))
            .await
            .unwrap();
        assert_eq!(code.user_id, user);
        assert_eq!(code.email, "alice@x.com");
        assert_eq!(code.code_hash, "hash");
        assert!(code.used_at.is_none());

        let found = repo.get_code_by_hash("hash").await.unwrap().expect("found");
        assert_eq!(found.id, code.id);
        assert!(repo.get_code_by_hash("missing").await.unwrap().is_none());
    }

    #[sqlx::test]
    async fn invalidate_codes_marks_outstanding_used(pool: PgPool) {
        let repo = PgEmailVerificationRepository::new(pool.clone());
        let user = Uuid::new_v4();
        insert_user(&pool, user, "alice").await;

        repo.create_code(user, "alice@x.com", "h1", Utc::now() + Duration::hours(1))
            .await
            .unwrap();
        repo.create_code(user, "alice@x.com", "h2", Utc::now() + Duration::hours(1))
            .await
            .unwrap();

        repo.invalidate_codes_for_email(user, "alice@x.com")
            .await
            .unwrap();

        assert!(
            repo.get_code_by_hash("h1")
                .await
                .unwrap()
                .unwrap()
                .used_at
                .is_some()
        );
        assert!(
            repo.get_code_by_hash("h2")
                .await
                .unwrap()
                .unwrap()
                .used_at
                .is_some()
        );
    }

    #[sqlx::test]
    async fn correct_code_consumes_and_creates_verified_email(pool: PgPool) {
        let repo = PgEmailVerificationRepository::new(pool.clone());
        let alice = Uuid::new_v4();
        insert_user(&pool, alice, "alice").await;

        repo.create_code(
            alice,
            "alice2@x.com",
            "hash",
            Utc::now() + Duration::hours(1),
        )
        .await
        .unwrap();

        let outcome = repo
            .verify_and_consume_email_code(alice, "alice2@x.com", "hash", MAX)
            .await
            .unwrap();

        let EmailCodeVerification::Success(row) = outcome else {
            panic!("expected success, got {outcome:?}");
        };
        assert_eq!(row.user_id, alice);
        assert_eq!(row.email, "alice2@x.com");
        assert!(row.is_verified);

        // The code is consumed.
        assert!(
            repo.get_code_by_hash("hash")
                .await
                .unwrap()
                .unwrap()
                .used_at
                .is_some()
        );
    }

    #[sqlx::test]
    async fn no_active_code_returns_no_active_code(pool: PgPool) {
        let repo = PgEmailVerificationRepository::new(pool.clone());
        let alice = Uuid::new_v4();
        insert_user(&pool, alice, "alice").await;

        let outcome = repo
            .verify_and_consume_email_code(alice, "alice2@x.com", "hash", MAX)
            .await
            .unwrap();
        assert!(matches!(outcome, EmailCodeVerification::NoActiveCode));
    }

    #[sqlx::test]
    async fn wrong_code_increments_then_locks_out(pool: PgPool) {
        let repo = PgEmailVerificationRepository::new(pool.clone());
        let alice = Uuid::new_v4();
        insert_user(&pool, alice, "alice").await;

        repo.create_code(
            alice,
            "alice2@x.com",
            "hash",
            Utc::now() + Duration::hours(1),
        )
        .await
        .unwrap();

        // The first MAX-1 wrong guesses are Invalid and increment the counter.
        for n in 1..MAX {
            let outcome = repo
                .verify_and_consume_email_code(alice, "alice2@x.com", "wrong", MAX)
                .await
                .unwrap();
            assert!(matches!(outcome, EmailCodeVerification::Invalid));
            assert_eq!(
                repo.get_code_by_hash("hash")
                    .await
                    .unwrap()
                    .unwrap()
                    .attempt_count,
                n
            );
        }

        // The MAX-th wrong guess burns the code.
        let outcome = repo
            .verify_and_consume_email_code(alice, "alice2@x.com", "wrong", MAX)
            .await
            .unwrap();
        assert!(matches!(outcome, EmailCodeVerification::AttemptsExhausted));
        assert!(
            repo.get_code_by_hash("hash")
                .await
                .unwrap()
                .unwrap()
                .used_at
                .is_some()
        );

        // Even the correct code no longer verifies once the code is burned.
        let outcome = repo
            .verify_and_consume_email_code(alice, "alice2@x.com", "hash", MAX)
            .await
            .unwrap();
        assert!(matches!(outcome, EmailCodeVerification::NoActiveCode));
    }
}
