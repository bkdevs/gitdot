use async_trait::async_trait;
use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::model::dag::Dag;

#[async_trait]
pub trait DagRepository: Send + Sync + Clone + 'static {
    async fn create(
        &self,
        repo_owner: &str,
        repo_name: &str,
        task_ids: &[Uuid],
    ) -> Result<Dag, Error>;
}

#[derive(Debug, Clone)]
pub struct DagRepositoryImpl {
    pool: PgPool,
}

impl DagRepositoryImpl {
    pub fn new(pool: PgPool) -> DagRepositoryImpl {
        DagRepositoryImpl { pool }
    }
}

#[async_trait]
impl DagRepository for DagRepositoryImpl {
    async fn create(
        &self,
        repo_owner: &str,
        repo_name: &str,
        task_ids: &[Uuid],
    ) -> Result<Dag, Error> {
        let dag = sqlx::query_as::<_, Dag>(
            r#"
            INSERT INTO dags (repo_owner, repo_name, task_ids)
            VALUES ($1, $2, $3)
            RETURNING id, repo_owner, repo_name, task_ids, created_at, updated_at
            "#,
        )
        .bind(repo_owner)
        .bind(repo_name)
        .bind(task_ids)
        .fetch_one(&self.pool)
        .await?;

        Ok(dag)
    }
}
