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
            RETURNING id, name, owner_id, owner_type, created_at
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
}
