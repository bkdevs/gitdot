use async_trait::async_trait;
use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::model::{Task, TaskStatus};

#[async_trait]
pub trait TaskRepository: Send + Sync + Clone + 'static {
    async fn get_by_id(&self, id: Uuid) -> Result<Task, Error>;

    async fn list_by_repo(&self, owner: &str, repo: &str) -> Result<Vec<Task>, Error>;

    async fn list_by_build_id(&self, build_id: Uuid) -> Result<Vec<Task>, Error>;

    async fn create(
        &self,
        id: Uuid,
        owner: &str,
        repo: &str,
        name: &str,
        command: &str,
        build_id: Uuid,
        status: TaskStatus,
        waits_for: &[Uuid],
    ) -> Result<Task, Error>;

    async fn update_task(&self, id: Uuid, status: TaskStatus) -> Result<Task, Error>;

    async fn claim_task(&self) -> Result<Option<Task>, Error>;

    async fn unblock_tasks(&self, build_id: Uuid) -> Result<Vec<Task>, Error>;
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
            SELECT id, repo_owner, repo_name, build_id, name, command, status, waits_for, created_at, updated_at
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
            SELECT id, repo_owner, repo_name, build_id, name, command, status, waits_for, created_at, updated_at
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

    async fn list_by_build_id(&self, build_id: Uuid) -> Result<Vec<Task>, Error> {
        let tasks = sqlx::query_as::<_, Task>(
            r#"
            SELECT id, repo_owner, repo_name, build_id, name, command, status, waits_for, created_at, updated_at
            FROM tasks WHERE build_id = $1
            ORDER BY created_at ASC
            "#,
        )
        .bind(build_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(tasks)
    }

    async fn create(
        &self,
        id: Uuid,
        owner: &str,
        repo: &str,
        name: &str,
        command: &str,
        build_id: Uuid,
        status: TaskStatus,
        waits_for: &[Uuid],
    ) -> Result<Task, Error> {
        let task = sqlx::query_as::<_, Task>(
            r#"
            INSERT INTO tasks (id, repo_owner, repo_name, name, command, build_id, status, waits_for)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, repo_owner, repo_name, build_id, name, command, status, waits_for, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(owner)
        .bind(repo)
        .bind(name)
        .bind(command)
        .bind(build_id)
        .bind(status)
        .bind(waits_for)
        .fetch_one(&self.pool)
        .await?;

        Ok(task)
    }

    async fn update_task(&self, id: Uuid, status: TaskStatus) -> Result<Task, Error> {
        let task = sqlx::query_as::<_, Task>(
            r#"
            UPDATE tasks SET status = $1, updated_at = NOW()
            WHERE id = $2
            RETURNING id, repo_owner, repo_name, build_id, name, command, status, waits_for, created_at, updated_at
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
            RETURNING id, repo_owner, repo_name, build_id, name, command, status, waits_for, created_at, updated_at
            "#,
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(task)
    }

    async fn unblock_tasks(&self, build_id: Uuid) -> Result<Vec<Task>, Error> {
        let tasks = sqlx::query_as::<_, Task>(
            r#"
            UPDATE tasks
            SET status = 'pending', updated_at = NOW()
            WHERE build_id = $1
              AND status = 'blocked'
              AND NOT EXISTS (
                SELECT 1 FROM unnest(waits_for) AS dep_id
                JOIN tasks t2 ON t2.id = dep_id
                WHERE t2.status != 'success'
              )
            RETURNING id, repo_owner, repo_name, build_id, name, command, status, waits_for, created_at, updated_at
            "#,
        )
        .bind(build_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(tasks)
    }
}
