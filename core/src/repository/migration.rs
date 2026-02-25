use async_trait::async_trait;
use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::model::{
    Migration, MigrationOriginService, MigrationRepository as MigrationRepositoryModel,
    MigrationRepositoryStatus, MigrationStatus, RepositoryOwnerType, RepositoryVisibility,
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

    async fn get(&self, author_id: Uuid, number: i32) -> Result<Option<Migration>, Error>;

    async fn list(&self, author_id: Uuid) -> Result<Vec<Migration>, Error>;

    async fn update_status(&self, id: Uuid, status: MigrationStatus) -> Result<Migration, Error>;

    async fn create_migration_repository(
        &self,
        migration_id: Uuid,
        origin_full_name: &str,
        destination_full_name: &str,
        visibility: &RepositoryVisibility,
    ) -> Result<MigrationRepositoryModel, Error>;

    async fn update_migration_repository_status(
        &self,
        id: Uuid,
        status: MigrationRepositoryStatus,
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
            RETURNING id, number, author_id, origin_service, origin, origin_type, destination, destination_type, status, created_at, updated_at, NULL AS repositories
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

    async fn get(&self, author_id: Uuid, number: i32) -> Result<Option<Migration>, Error> {
        sqlx::query_as::<_, Migration>(
            r#"
            SELECT m.id, m.number, m.author_id, m.origin_service, m.origin, m.origin_type,
                   m.destination, m.destination_type, m.status, m.created_at, m.updated_at,
                   COALESCE(
                       (SELECT json_agg(json_build_object(
                           'id', mr.id,
                           'migration_id', mr.migration_id,
                           'origin_full_name', mr.origin_full_name,
                           'destination_full_name', mr.destination_full_name,
                           'visibility', mr.visibility,
                           'status', mr.status,
                           'error', mr.error,
                           'created_at', mr.created_at,
                           'updated_at', mr.updated_at
                       ) ORDER BY mr.created_at ASC)
                       FROM migration_repositories mr WHERE mr.migration_id = m.id),
                       '[]'::json
                   ) AS repositories
            FROM migrations m
            WHERE m.author_id = $1 AND m.number = $2
            "#,
        )
        .bind(author_id)
        .bind(number)
        .fetch_optional(&self.pool)
        .await
    }

    async fn list(&self, author_id: Uuid) -> Result<Vec<Migration>, Error> {
        sqlx::query_as::<_, Migration>(
            r#"
            SELECT m.id, m.number, m.author_id, m.origin_service, m.origin, m.origin_type,
                   m.destination, m.destination_type, m.status, m.created_at, m.updated_at,
                   COALESCE(
                       (SELECT json_agg(json_build_object(
                           'id', mr.id,
                           'migration_id', mr.migration_id,
                           'origin_full_name', mr.origin_full_name,
                           'destination_full_name', mr.destination_full_name,
                           'visibility', mr.visibility,
                           'status', mr.status,
                           'error', mr.error,
                           'created_at', mr.created_at,
                           'updated_at', mr.updated_at
                       ) ORDER BY mr.created_at ASC)
                       FROM migration_repositories mr WHERE mr.migration_id = m.id),
                       '[]'::json
                   ) AS repositories
            FROM migrations m
            WHERE m.author_id = $1
            ORDER BY m.created_at DESC
            "#,
        )
        .bind(author_id)
        .fetch_all(&self.pool)
        .await
    }

    async fn update_status(&self, id: Uuid, status: MigrationStatus) -> Result<Migration, Error> {
        sqlx::query_as::<_, Migration>(
            r#"
            UPDATE migrations SET status = $2, updated_at = NOW()
            WHERE id = $1
            RETURNING id, number, author_id, origin_service, origin, origin_type, destination, destination_type, status, created_at, updated_at, NULL AS repositories
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
        visibility: &RepositoryVisibility,
    ) -> Result<MigrationRepositoryModel, Error> {
        sqlx::query_as::<_, MigrationRepositoryModel>(
            r#"
            INSERT INTO migration_repositories (migration_id, origin_full_name, destination_full_name, visibility)
            VALUES ($1, $2, $3, $4)
            RETURNING id, migration_id, origin_full_name, destination_full_name, visibility, status, error, created_at, updated_at
            "#,
        )
        .bind(migration_id)
        .bind(origin_full_name)
        .bind(destination_full_name)
        .bind(visibility)
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
            RETURNING id, migration_id, origin_full_name, destination_full_name, visibility, status, error, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(status)
        .bind(error)
        .fetch_one(&self.pool)
        .await
    }
}
