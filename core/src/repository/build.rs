use std::collections::HashMap;

use async_trait::async_trait;
use sqlx::{Error, PgPool, types::Json};
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
        task_dependencies: &HashMap<Uuid, Vec<Uuid>>,
    ) -> Result<Build, Error>;

    async fn update_task_dependencies(
        &self,
        id: Uuid,
        task_dependencies: &HashMap<Uuid, Vec<Uuid>>,
    ) -> Result<Build, Error>;
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
        task_dependencies: &HashMap<Uuid, Vec<Uuid>>,
    ) -> Result<Build, Error> {
        let build = sqlx::query_as::<_, Build>(
            r#"
            INSERT INTO builds (repo_owner, repo_name, trigger, commit_sha, task_dependencies)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, repo_owner, repo_name, trigger, commit_sha, task_dependencies, created_at, updated_at
            "#,
        )
        .bind(repo_owner)
        .bind(repo_name)
        .bind(trigger)
        .bind(commit_sha)
        .bind(Json(task_dependencies))
        .fetch_one(&self.pool)
        .await?;

        Ok(build)
    }

    async fn update_task_dependencies(
        &self,
        id: Uuid,
        task_dependencies: &HashMap<Uuid, Vec<Uuid>>,
    ) -> Result<Build, Error> {
        let build = sqlx::query_as::<_, Build>(
            r#"
            UPDATE builds SET task_dependencies = $1, updated_at = NOW()
            WHERE id = $2
            RETURNING id, repo_owner, repo_name, trigger, commit_sha, task_dependencies, created_at, updated_at
            "#,
        )
        .bind(Json(task_dependencies))
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(build)
    }
}
