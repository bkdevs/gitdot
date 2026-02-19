use async_trait::async_trait;
use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::model::task::{Task, TaskStatus};

#[async_trait]
pub trait TaskRepository: Send + Sync + Clone + 'static {
    async fn update(&self, id: Uuid, status: &TaskStatus) -> Result<Task, Error>;
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
    async fn update(&self, id: Uuid, status: &TaskStatus) -> Result<Task, Error> {
        let task = sqlx::query_as::<_, Task>(
            r#"
            UPDATE tasks SET status = $1, updated_at = NOW() WHERE id = $2
            RETURNING id, repo_owner, repo_name, script, status, created_at, updated_at
            "#,
        )
        .bind(status)
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(task)
    }
}
