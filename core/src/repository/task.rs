use async_trait::async_trait;
use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::model::{Task, TaskStatus};

#[async_trait]
pub trait TaskRepository: Send + Sync + Clone + 'static {
    async fn get_by_id(&self, id: Uuid) -> Result<Task, Error>;
    async fn list_by_repo(&self, owner: &str, repo: &str) -> Result<Vec<Task>, Error>;
    async fn create(
        &self,
        owner: &str,
        repo: &str,
        script: &str,
        dag_id: Uuid,
    ) -> Result<Task, Error>;
    async fn update_task(&self, id: Uuid, status: TaskStatus) -> Result<Task, Error>;
    async fn claim_task(&self) -> Result<Option<Task>, Error>;
}

#[derive(Debug, Clone)]
pub struct TaskRepositoryImpl {
    pool: PgPool,
}

impl TaskRepositoryImpl {
    pub fn new(pool: PgPool) -> TaskRepositoryImpl {
        TaskRepositoryImpl { pool }
    }
}

#[async_trait]
impl TaskRepository for TaskRepositoryImpl {
    async fn get_by_id(&self, id: Uuid) -> Result<Task, Error> {
        let task = sqlx::query_as::<_, Task>(
            r#"
            SELECT id, repo_owner, repo_name, dag_id, script, status, created_at, updated_at
            FROM tasks WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(task)
    }

    async fn list_by_repo(&self, owner: &str, repo: &str) -> Result<Vec<Task>, Error> {
        let tasks = sqlx::query_as::<_, Task>(
            r#"
            SELECT id, repo_owner, repo_name, dag_id, script, status, created_at, updated_at
            FROM tasks WHERE repo_owner = $1 AND repo_name = $2
            ORDER BY created_at ASC
            "#,
        )
        .bind(owner)
        .bind(repo)
        .fetch_all(&self.pool)
        .await?;

        Ok(tasks)
    }

    async fn create(
        &self,
        owner: &str,
        repo: &str,
        script: &str,
        dag_id: Uuid,
    ) -> Result<Task, Error> {
        let task = sqlx::query_as::<_, Task>(
            r#"
            INSERT INTO tasks (repo_owner, repo_name, script, dag_id)
            VALUES ($1, $2, $3, $4)
            RETURNING id, repo_owner, repo_name, dag_id, script, status, created_at, updated_at
            "#,
        )
        .bind(owner)
        .bind(repo)
        .bind(script)
        .bind(dag_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(task)
    }

    async fn update_task(&self, id: Uuid, status: TaskStatus) -> Result<Task, Error> {
        let task = sqlx::query_as::<_, Task>(
            r#"
            UPDATE tasks SET status = $1, updated_at = NOW()
            WHERE id = $2
            RETURNING id, repo_owner, repo_name, dag_id, script, status, created_at, updated_at
            "#,
        )
        .bind(status)
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(task)
    }

    async fn claim_task(&self) -> Result<Option<Task>, Error> {
        let task = sqlx::query_as::<_, Task>(
            r#"
            UPDATE tasks SET status = 'assigned', updated_at = NOW()
            WHERE id = (
                SELECT id FROM tasks WHERE status = 'pending'
                ORDER BY created_at ASC
                LIMIT 1
                FOR UPDATE SKIP LOCKED
            )
            RETURNING id, repo_owner, repo_name, dag_id, script, status, created_at, updated_at
            "#,
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(task)
    }
}
