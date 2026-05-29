use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{error::DatabaseError, model::EmailVerificationCode};

#[async_trait]
pub trait EmailVerificationRepository: Send + Sync + Clone + 'static {
    async fn create_code(
        &self,
        user_email_id: Uuid,
        code_hash: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<EmailVerificationCode, DatabaseError>;

    async fn get_code_by_hash(
        &self,
        code_hash: &str,
    ) -> Result<Option<EmailVerificationCode>, DatabaseError>;

    async fn invalidate_codes_for_email(&self, user_email_id: Uuid) -> Result<(), DatabaseError>;

    async fn mark_code_used_and_verify_email(
        &self,
        code_id: Uuid,
        user_email_id: Uuid,
    ) -> Result<(), DatabaseError>;
}

#[derive(Debug, Clone)]
pub struct EmailVerificationRepositoryImpl {
    pool: PgPool,
}

impl EmailVerificationRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl EmailVerificationRepository for EmailVerificationRepositoryImpl {
    async fn create_code(
        &self,
        user_email_id: Uuid,
        code_hash: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<EmailVerificationCode, DatabaseError> {
        let code = sqlx::query_as::<_, EmailVerificationCode>(
            r#"
            INSERT INTO auth.email_verification_codes (user_email_id, code_hash, expires_at)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
        )
        .bind(user_email_id)
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

    async fn invalidate_codes_for_email(&self, user_email_id: Uuid) -> Result<(), DatabaseError> {
        sqlx::query(
            r#"
            UPDATE auth.email_verification_codes
            SET used_at = NOW()
            WHERE user_email_id = $1 AND used_at IS NULL
            "#,
        )
        .bind(user_email_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn mark_code_used_and_verify_email(
        &self,
        code_id: Uuid,
        user_email_id: Uuid,
    ) -> Result<(), DatabaseError> {
        let mut tx = self.pool.begin().await?;

        sqlx::query(
            r#"
            UPDATE auth.email_verification_codes SET used_at = NOW() WHERE id = $1
            "#,
        )
        .bind(code_id)
        .execute(&mut *tx)
        .await?;

        // Clean up squatter rows: any other unverified rows claiming the same
        // email are bogus once one user verifies, so drop them. Done before the
        // UPDATE so the partial unique index on `(email) WHERE is_verified`
        // never sees two conflicting rows in flight.
        sqlx::query(
            r#"
            DELETE FROM core.user_emails
            WHERE email = (SELECT email FROM core.user_emails WHERE id = $1)
              AND id != $1
              AND NOT is_verified
            "#,
        )
        .bind(user_email_id)
        .execute(&mut *tx)
        .await?;

        sqlx::query(
            r#"
            UPDATE core.user_emails
            SET is_verified = TRUE, verified_at = COALESCE(verified_at, NOW())
            WHERE id = $1
            "#,
        )
        .bind(user_email_id)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(())
    }
}

#[cfg(all(test, feature = "db-tests"))]
mod tests {
    use chrono::{Duration, Utc};
    use sqlx::PgPool;
    use uuid::Uuid;

    use super::{EmailVerificationRepository, EmailVerificationRepositoryImpl};
    use crate::repository::test_common::insert_user;

    async fn insert_user_email(pool: &PgPool, id: Uuid, user_id: Uuid, email: &str) {
        sqlx::query(
            "INSERT INTO core.user_emails (id, user_id, email, is_primary, is_verified)
             VALUES ($1, $2, $3, FALSE, FALSE)",
        )
        .bind(id)
        .bind(user_id)
        .bind(email)
        .execute(pool)
        .await
        .unwrap();
    }

    async fn is_verified(pool: &PgPool, email_id: Uuid) -> bool {
        sqlx::query_scalar::<_, bool>("SELECT is_verified FROM core.user_emails WHERE id = $1")
            .bind(email_id)
            .fetch_one(pool)
            .await
            .unwrap()
    }

    async fn email_count(pool: &PgPool, email: &str) -> i64 {
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM core.user_emails WHERE email = $1")
            .bind(email)
            .fetch_one(pool)
            .await
            .unwrap()
    }

    #[sqlx::test]
    async fn create_and_get_code(pool: PgPool) {
        let repo = EmailVerificationRepositoryImpl::new(pool.clone());
        let user = Uuid::new_v4();
        let email_id = Uuid::new_v4();
        insert_user(&pool, user, "alice").await;
        insert_user_email(&pool, email_id, user, "alice@x.com").await;

        let code = repo
            .create_code(email_id, "hash", Utc::now() + Duration::hours(1))
            .await
            .unwrap();
        assert_eq!(code.user_email_id, email_id);
        assert_eq!(code.code_hash, "hash");
        assert!(code.used_at.is_none());

        let found = repo.get_code_by_hash("hash").await.unwrap().expect("found");
        assert_eq!(found.id, code.id);
        assert!(repo.get_code_by_hash("missing").await.unwrap().is_none());
    }

    #[sqlx::test]
    async fn invalidate_codes_marks_outstanding_used(pool: PgPool) {
        let repo = EmailVerificationRepositoryImpl::new(pool.clone());
        let user = Uuid::new_v4();
        let email_id = Uuid::new_v4();
        insert_user(&pool, user, "alice").await;
        insert_user_email(&pool, email_id, user, "alice@x.com").await;

        repo.create_code(email_id, "h1", Utc::now() + Duration::hours(1))
            .await
            .unwrap();
        repo.create_code(email_id, "h2", Utc::now() + Duration::hours(1))
            .await
            .unwrap();

        repo.invalidate_codes_for_email(email_id).await.unwrap();

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
    async fn mark_code_used_verifies_and_removes_squatters(pool: PgPool) {
        let repo = EmailVerificationRepositoryImpl::new(pool.clone());
        let alice = Uuid::new_v4();
        let bob = Uuid::new_v4();
        let alice_email = Uuid::new_v4();
        let bob_email = Uuid::new_v4();
        insert_user(&pool, alice, "alice").await;
        insert_user(&pool, bob, "bob").await;
        // Both unverified rows claim the same address.
        insert_user_email(&pool, alice_email, alice, "shared@x.com").await;
        insert_user_email(&pool, bob_email, bob, "shared@x.com").await;

        let code = repo
            .create_code(alice_email, "hash", Utc::now() + Duration::hours(1))
            .await
            .unwrap();

        repo.mark_code_used_and_verify_email(code.id, alice_email)
            .await
            .unwrap();

        // The code is consumed and alice's email becomes verified.
        assert!(
            repo.get_code_by_hash("hash")
                .await
                .unwrap()
                .unwrap()
                .used_at
                .is_some()
        );
        assert!(is_verified(&pool, alice_email).await);

        // The competing unverified squatter row is removed.
        assert_eq!(email_count(&pool, "shared@x.com").await, 1);
    }
}
