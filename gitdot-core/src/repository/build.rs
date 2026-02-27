use async_trait::async_trait;
use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::model::Build;

#[async_trait]
pub trait BuildRepository: Send + Sync + Clone + 'static {
    async fn create(
        &self,
        repository_id: Uuid,
        trigger: &str,
        commit_sha: &str,
    ) -> Result<Build, Error>;

    async fn get(&self, repository_id: Uuid, number: i32) -> Result<Option<Build>, Error>;

    async fn list_by_repo(&self, repository_id: Uuid) -> Result<Vec<Build>, Error>;
}

#[derive(Debug, Clone)]
pub struct BuildRepositoryImpl {
    pool: PgPool,
}

impl BuildRepositoryImpl {
    pub fn new(pool: PgPool) -> BuildRepositoryImpl {
        BuildRepositoryImpl { pool }
    }
}

#[async_trait]
impl BuildRepository for BuildRepositoryImpl {
    async fn create(
        &self,
        repository_id: Uuid,
        trigger: &str,
        commit_sha: &str,
    ) -> Result<Build, Error> {
        let build = sqlx::query_as::<_, Build>(
            r#"
            INSERT INTO builds (repository_id, trigger, commit_sha, number)
            VALUES ($1, $2, $3, COALESCE((SELECT MAX(number) FROM builds WHERE repository_id = $1), 0) + 1)
            RETURNING id, number, repository_id, trigger, commit_sha, status, created_at, updated_at
            "#,
        )
        .bind(repository_id)
        .bind(trigger)
        .bind(commit_sha)
        .fetch_one(&self.pool)
        .await?;

        Ok(build)
    }

    async fn get(&self, repository_id: Uuid, number: i32) -> Result<Option<Build>, Error> {
        let build = sqlx::query_as::<_, Build>(
            r#"
            SELECT id, number, repository_id, trigger, commit_sha, status, created_at, updated_at
            FROM builds WHERE repository_id = $1 AND number = $2
            "#,
        )
        .bind(repository_id)
        .bind(number)
        .fetch_optional(&self.pool)
        .await?;

        Ok(build)
    }

    async fn list_by_repo(&self, repository_id: Uuid) -> Result<Vec<Build>, Error> {
        let builds = sqlx::query_as::<_, Build>(
            r#"
            SELECT id, number, repository_id, trigger, commit_sha, status, created_at, updated_at
            FROM builds WHERE repository_id = $1
            ORDER BY created_at ASC
            "#,
        )
        .bind(repository_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(builds)
    }
}
