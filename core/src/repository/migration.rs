use async_trait::async_trait;
use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::model::{
    Migration, MigrationOrigin, MigrationRepository as MigrationRepositoryModel,
    MigrationRepositoryStatus, MigrationStatus,
};

#[async_trait]
pub trait MigrationRepository: Send + Sync + Clone + 'static {
    async fn create(&self, author_id: Uuid, origin: MigrationOrigin) -> Result<Migration, Error>;

    async fn update_status(&self, id: Uuid, status: MigrationStatus) -> Result<Migration, Error>;

    async fn create_migration_repository(
        &self,
        migration_id: Uuid,
        full_name: &str,
    ) -> Result<MigrationRepositoryModel, Error>;

    async fn update_migration_repository_status(
        &self,
        id: Uuid,
        status: MigrationRepositoryStatus,
        repository_id: Option<Uuid>,
        error: Option<&str>,
    ) -> Result<MigrationRepositoryModel, Error>;
}

#[derive(Debug, Clone)]
pub struct MigrationRepositoryImpl {
    pool: PgPool,
}

impl MigrationRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl MigrationRepository for MigrationRepositoryImpl {
    async fn create(&self, author_id: Uuid, origin: MigrationOrigin) -> Result<Migration, Error> {
        sqlx::query_as::<_, Migration>(
            r#"
            INSERT INTO migrations (author_id, origin)
            VALUES ($1, $2)
            RETURNING id, author_id, origin, status, created_at, updated_at
            "#,
        )
        .bind(author_id)
        .bind(origin)
        .fetch_one(&self.pool)
        .await
    }

    async fn update_status(&self, id: Uuid, status: MigrationStatus) -> Result<Migration, Error> {
        sqlx::query_as::<_, Migration>(
            r#"
            UPDATE migrations SET status = $2, updated_at = NOW()
            WHERE id = $1
            RETURNING id, author_id, origin, status, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(status)
        .fetch_one(&self.pool)
        .await
    }

    async fn create_migration_repository(
        &self,
        migration_id: Uuid,
        full_name: &str,
    ) -> Result<MigrationRepositoryModel, Error> {
        sqlx::query_as::<_, MigrationRepositoryModel>(
            r#"
            INSERT INTO migration_repositories (migration_id, full_name)
            VALUES ($1, $2)
            RETURNING id, migration_id, repository_id, full_name, status, error, created_at, updated_at
            "#,
        )
        .bind(migration_id)
        .bind(full_name)
        .fetch_one(&self.pool)
        .await
    }

    async fn update_migration_repository_status(
        &self,
        id: Uuid,
        status: MigrationRepositoryStatus,
        repository_id: Option<Uuid>,
        error: Option<&str>,
    ) -> Result<MigrationRepositoryModel, Error> {
        sqlx::query_as::<_, MigrationRepositoryModel>(
            r#"
            UPDATE migration_repositories
            SET status = $2, repository_id = COALESCE($3, repository_id), error = $4, updated_at = NOW()
            WHERE id = $1
            RETURNING id, migration_id, repository_id, full_name, status, error, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(status)
        .bind(repository_id)
        .bind(error)
        .fetch_one(&self.pool)
        .await
    }
}
