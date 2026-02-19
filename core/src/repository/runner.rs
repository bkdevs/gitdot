use async_trait::async_trait;
use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::model::{Runner, RunnerOwnerType};

#[async_trait]
pub trait RunnerRepository: Send + Sync + Clone + 'static {
    async fn create(
        &self,
        name: &str,
        owner_id: Uuid,
        owner_type: &RunnerOwnerType,
    ) -> Result<Runner, Error>;
    async fn delete(&self, id: Uuid) -> Result<(), Error>;
    async fn touch(&self, id: Uuid) -> Result<(), Error>;
    async fn get(&self, owner_name: &str, name: &str) -> Result<Option<Runner>, Error>;
}

#[derive(Debug, Clone)]
pub struct RunnerRepositoryImpl {
    pool: PgPool,
}

impl RunnerRepositoryImpl {
    pub fn new(pool: PgPool) -> RunnerRepositoryImpl {
        RunnerRepositoryImpl { pool }
    }
}

#[async_trait]
impl RunnerRepository for RunnerRepositoryImpl {
    async fn create(
        &self,
        name: &str,
        owner_id: Uuid,
        owner_type: &RunnerOwnerType,
    ) -> Result<Runner, Error> {
        let runner = sqlx::query_as::<_, Runner>(
            r#"
            INSERT INTO runners (name, owner_id, owner_type)
            VALUES ($1, $2, $3)
            RETURNING id, name, owner_id, owner_type, last_verified, created_at
            "#,
        )
        .bind(name)
        .bind(owner_id)
        .bind(owner_type)
        .fetch_one(&self.pool)
        .await?;

        Ok(runner)
    }

    async fn delete(&self, id: Uuid) -> Result<(), Error> {
        let result = sqlx::query("DELETE FROM runners WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(Error::RowNotFound);
        }

        Ok(())
    }

    async fn touch(&self, id: Uuid) -> Result<(), Error> {
        let result = sqlx::query("UPDATE runners SET last_verified = NOW() WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(Error::RowNotFound);
        }

        Ok(())
    }

    async fn get(&self, owner_name: &str, name: &str) -> Result<Option<Runner>, Error> {
        let runner = sqlx::query_as::<_, Runner>(
            r#"
            SELECT r.id, r.name, r.owner_id, r.owner_type, r.last_verified, r.created_at
            FROM runners r
            LEFT JOIN users u ON r.owner_id = u.id AND r.owner_type = 'user'
            LEFT JOIN organizations o ON r.owner_id = o.id AND r.owner_type = 'organization'
            WHERE r.name = $2
              AND (u.name = $1 OR o.name = $1)
            "#,
        )
        .bind(owner_name)
        .bind(name)
        .fetch_optional(&self.pool)
        .await?;

        Ok(runner)
    }
}
