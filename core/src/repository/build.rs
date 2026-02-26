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

    async fn get(&self, id: Uuid) -> Result<Build, Error>;

    async fn get_by_number(
        &self,
        owner: &str,
        repo: &str,
        number: i32,
    ) -> Result<Option<Build>, Error>;

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
            INSERT INTO builds (repo_owner, repo_name, trigger, commit_sha, number)
            VALUES ($1, $2, $3, $4, COALESCE((SELECT MAX(number) FROM builds WHERE repo_owner = $1 AND repo_name = $2), 0) + 1)
            RETURNING id, number, repo_owner, repo_name, trigger, commit_sha, created_at, updated_at
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

    async fn get(&self, id: Uuid) -> Result<Build, Error> {
        let build = sqlx::query_as::<_, Build>(
            r#"
            SELECT id, number, repo_owner, repo_name, trigger, commit_sha, created_at, updated_at
            FROM builds WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(build)
    }

    async fn get_by_number(
        &self,
        owner: &str,
        repo: &str,
        number: i32,
    ) -> Result<Option<Build>, Error> {
        let build = sqlx::query_as::<_, Build>(
            r#"
            SELECT id, number, repo_owner, repo_name, trigger, commit_sha, created_at, updated_at
            FROM builds WHERE repo_owner = $1 AND repo_name = $2 AND number = $3
            "#,
        )
        .bind(owner)
        .bind(repo)
        .bind(number)
        .fetch_optional(&self.pool)
        .await?;

        Ok(build)
    }

    async fn list_by_repo(&self, owner: &str, repo: &str) -> Result<Vec<Build>, Error> {
        let builds = sqlx::query_as::<_, Build>(
            r#"
            SELECT id, number, repo_owner, repo_name, trigger, commit_sha, created_at, updated_at
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
