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
