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
        task_dependencies: &HashMap<Uuid, Vec<Uuid>>,
    ) -> Result<Build, Error> {
        let build = sqlx::query_as::<_, Build>(
            r#"
            INSERT INTO builds (repo_owner, repo_name, task_dependencies)
            VALUES ($1, $2, $3)
            RETURNING id, repo_owner, repo_name, task_dependencies, created_at, updated_at
            "#,
        )
        .bind(repo_owner)
        .bind(repo_name)
        .bind(Json(task_dependencies))
        .fetch_one(&self.pool)
        .await?;

        Ok(build)
    }
}
