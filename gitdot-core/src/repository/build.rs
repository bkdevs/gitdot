use async_trait::async_trait;
use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::model::{Build, BuildTrigger, BuildWithStats};

#[async_trait]
pub trait BuildRepository: Send + Sync + Clone + 'static {
    async fn create(
        &self,
        repository_id: Uuid,
        trigger: BuildTrigger,
        commit_sha: &str,
    ) -> Result<Build, Error>;

    async fn get(&self, repository_id: Uuid, number: i32) -> Result<Option<Build>, Error>;

    async fn list_by_repo(&self, repository_id: Uuid) -> Result<Vec<BuildWithStats>, Error>;
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
        trigger: BuildTrigger,
        commit_sha: &str,
    ) -> Result<Build, Error> {
        let build = sqlx::query_as::<_, Build>(
            r#"
            INSERT INTO builds (repository_id, trigger, commit_sha, number)
            VALUES ($1, $2, $3, COALESCE((SELECT MAX(number) FROM builds WHERE repository_id = $1), 0) + 1)
            RETURNING id, number, repository_id, trigger, commit_sha, status, created_at
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
            SELECT id, number, repository_id, trigger, commit_sha, status, created_at
            FROM builds WHERE repository_id = $1 AND number = $2
            "#,
        )
        .bind(repository_id)
        .bind(number)
        .fetch_optional(&self.pool)
        .await?;

        Ok(build)
    }

    async fn list_by_repo(&self, repository_id: Uuid) -> Result<Vec<BuildWithStats>, Error> {
        let builds = sqlx::query_as::<_, BuildWithStats>(
            r#"
            SELECT
                b.id, b.number, b.repository_id, b.trigger, b.commit_sha,
                CASE
                    WHEN COUNT(t.id) = 0 THEN 'running'::build_status
                    WHEN COUNT(t.id) FILTER (WHERE t.status = 'failure') > 0 THEN 'failure'::build_status
                    WHEN COUNT(t.id) = COUNT(t.id) FILTER (WHERE t.status = 'success') THEN 'success'::build_status
                    ELSE 'running'::build_status
                END AS status,
                CAST(COUNT(t.id) AS INT) AS total_tasks,
                CAST(COUNT(t.id) FILTER (WHERE t.status = 'success') AS INT) AS completed_tasks,
                b.created_at,
                COALESCE(MAX(t.updated_at), b.created_at) AS updated_at
            FROM builds b
            LEFT JOIN tasks t ON t.build_id = b.id
            WHERE b.repository_id = $1
            GROUP BY b.id, b.number, b.repository_id, b.trigger, b.commit_sha, b.created_at
            ORDER BY b.created_at ASC
            "#,
        )
        .bind(repository_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(builds)
    }
}
