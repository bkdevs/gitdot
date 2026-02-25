use async_trait::async_trait;
use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::model::Build;

#[async_trait]
pub trait BuildRepository: Send + Sync + Clone + 'static {
    async fn create(
        &self,
        repo_owner: &str,
        repo_name: &str,
        trigger: &str,
        commit_sha: &str,
    ) -> Result<Build, Error>;

    async fn get_by_id(&self, id: Uuid) -> Result<Build, Error>;

    async fn list_by_repo(&self, owner: &str, repo: &str) -> Result<Vec<Build>, Error>;
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
        repo_owner: &str,
        repo_name: &str,
        trigger: &str,
        commit_sha: &str,
    ) -> Result<Build, Error> {
        let build = sqlx::query_as::<_, Build>(
            r#"
            INSERT INTO builds (repo_owner, repo_name, trigger, commit_sha)
            VALUES ($1, $2, $3, $4)
            RETURNING id, repo_owner, repo_name, trigger, commit_sha, created_at, updated_at
            "#,
        )
        .bind(repo_owner)
        .bind(repo_name)
        .bind(trigger)
        .bind(commit_sha)
        .fetch_one(&self.pool)
        .await?;

        Ok(build)
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Build, Error> {
        let build = sqlx::query_as::<_, Build>(
            r#"
            SELECT id, repo_owner, repo_name, trigger, commit_sha, created_at, updated_at
            FROM builds WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(build)
    }

    async fn list_by_repo(&self, owner: &str, repo: &str) -> Result<Vec<Build>, Error> {
        let builds = sqlx::query_as::<_, Build>(
            r#"
            SELECT id, repo_owner, repo_name, trigger, commit_sha, created_at, updated_at
            FROM builds WHERE repo_owner = $1 AND repo_name = $2
            ORDER BY created_at ASC
            "#,
        )
        .bind(owner)
        .bind(repo)
        .fetch_all(&self.pool)
        .await?;

        Ok(builds)
    }
}
