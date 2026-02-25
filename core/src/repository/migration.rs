use async_trait::async_trait;
use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::model::{
    Migration, MigrationOriginService, MigrationRepository as MigrationRepositoryModel,
    MigrationRepositoryStatus, MigrationStatus, RepositoryOwnerType,
};

#[async_trait]
pub trait MigrationRepository: Send + Sync + Clone + 'static {
    async fn create(
        &self,
        author_id: Uuid,
        origin_service: MigrationOriginService,
        origin: &str,
        origin_type: &RepositoryOwnerType,
        destination: &str,
        destination_type: &RepositoryOwnerType,
    ) -> Result<Migration, Error>;

    async fn update_status(&self, id: Uuid, status: MigrationStatus) -> Result<Migration, Error>;

    async fn create_migration_repository(
        &self,
        migration_id: Uuid,
        origin_full_name: &str,
        destination_full_name: &str,
    ) -> Result<MigrationRepositoryModel, Error>;

    async fn update_migration_repository_status(
        &self,
        id: Uuid,
        status: MigrationRepositoryStatus,
        error: Option<&str>,
    ) -> Result<MigrationRepositoryModel, Error>;

    async fn get_by_author_and_number(
        &self,
        author_id: Uuid,
        number: i32,
    ) -> Result<Option<Migration>, Error>;

    async fn list_by_author(&self, author_id: Uuid) -> Result<Vec<Migration>, Error>;

    async fn list_migration_repositories(
        &self,
        migration_id: Uuid,
    ) -> Result<Vec<MigrationRepositoryModel>, Error>;
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
    async fn create(
        &self,
        author_id: Uuid,
        origin_service: MigrationOriginService,
        origin: &str,
        origin_type: &RepositoryOwnerType,
        destination: &str,
        destination_type: &RepositoryOwnerType,
    ) -> Result<Migration, Error> {
        sqlx::query_as::<_, Migration>(
            r#"
            INSERT INTO migrations (number, author_id, origin_service, origin, origin_type, destination, destination_type)
            VALUES (
                COALESCE((SELECT MAX(number) FROM migrations WHERE author_id = $1), 0) + 1,
                $1, $2, $3, $4, $5, $6
            )
            RETURNING id, number, author_id, origin_service, origin, origin_type, destination, destination_type, status, created_at, updated_at
            "#,
        )
        .bind(author_id)
        .bind(origin_service)
        .bind(origin)
        .bind(origin_type)
        .bind(destination)
        .bind(destination_type)
        .fetch_one(&self.pool)
        .await
    }

    async fn update_status(&self, id: Uuid, status: MigrationStatus) -> Result<Migration, Error> {
        sqlx::query_as::<_, Migration>(
            r#"
            UPDATE migrations SET status = $2, updated_at = NOW()
            WHERE id = $1
            RETURNING id, number, author_id, origin_service, origin, origin_type, destination, destination_type, status, created_at, updated_at
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
        origin_full_name: &str,
        destination_full_name: &str,
    ) -> Result<MigrationRepositoryModel, Error> {
        sqlx::query_as::<_, MigrationRepositoryModel>(
            r#"
            INSERT INTO migration_repositories (migration_id, origin_full_name, destination_full_name)
            VALUES ($1, $2, $3)
            RETURNING id, migration_id, origin_full_name, destination_full_name, status, error, created_at, updated_at
            "#,
        )
        .bind(migration_id)
        .bind(origin_full_name)
        .bind(destination_full_name)
        .fetch_one(&self.pool)
        .await
    }

    async fn update_migration_repository_status(
        &self,
        id: Uuid,
        status: MigrationRepositoryStatus,
        error: Option<&str>,
    ) -> Result<MigrationRepositoryModel, Error> {
        sqlx::query_as::<_, MigrationRepositoryModel>(
            r#"
            UPDATE migration_repositories
            SET status = $2, error = $3, updated_at = NOW()
            WHERE id = $1
            RETURNING id, migration_id, origin_full_name, destination_full_name, status, error, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(status)
        .bind(error)
        .fetch_one(&self.pool)
        .await
    }

    async fn get_by_author_and_number(
        &self,
        author_id: Uuid,
        number: i32,
    ) -> Result<Option<Migration>, Error> {
        sqlx::query_as::<_, Migration>(
            r#"
            SELECT id, number, author_id, origin_service, origin, origin_type, destination, destination_type, status, created_at, updated_at
            FROM migrations
            WHERE author_id = $1 AND number = $2
            "#,
        )
        .bind(author_id)
        .bind(number)
        .fetch_optional(&self.pool)
        .await
    }

    async fn list_by_author(&self, author_id: Uuid) -> Result<Vec<Migration>, Error> {
        sqlx::query_as::<_, Migration>(
            r#"
            SELECT id, number, author_id, origin_service, origin, origin_type, destination, destination_type, status, created_at, updated_at
            FROM migrations
            WHERE author_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(author_id)
        .fetch_all(&self.pool)
        .await
    }

    async fn list_migration_repositories(
        &self,
        migration_id: Uuid,
    ) -> Result<Vec<MigrationRepositoryModel>, Error> {
        sqlx::query_as::<_, MigrationRepositoryModel>(
            r#"
            SELECT id, migration_id, origin_full_name, destination_full_name, status, error, created_at, updated_at
            FROM migration_repositories
            WHERE migration_id = $1
            ORDER BY created_at ASC
            "#,
        )
        .bind(migration_id)
        .fetch_all(&self.pool)
        .await
    }
}
