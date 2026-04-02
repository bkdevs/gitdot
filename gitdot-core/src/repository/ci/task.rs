use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::DatabaseError,
    model::{Task, TaskStatus},
};

#[async_trait]
pub trait TaskRepository: Send + Sync + Clone + 'static {
    async fn create(
        &self,
        id: Uuid,
        repository_id: Uuid,
        name: &str,
        command: &str,
        build_id: Uuid,
        s2_uri: &str,
        status: TaskStatus,
        waits_for: &[Uuid],
    ) -> Result<Task, DatabaseError>;

    async fn get_by_id(&self, id: Uuid) -> Result<Option<Task>, DatabaseError>;

    async fn list_by_build_id(&self, build_id: Uuid) -> Result<Vec<Task>, DatabaseError>;

    async fn update_task(&self, id: Uuid, status: TaskStatus) -> Result<Task, DatabaseError>;

    async fn claim_task(
        &self,
        runner_id: Uuid,
        repository_ids: &[Uuid],
    ) -> Result<Option<Task>, DatabaseError>;

    async fn unblock_tasks(&self, build_id: Uuid) -> Result<Vec<Task>, DatabaseError>;
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

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl TaskRepository for TaskRepositoryImpl {
    async fn create(
        &self,
        id: Uuid,
        repository_id: Uuid,
        name: &str,
        command: &str,
        build_id: Uuid,
        s2_uri: &str,
        status: TaskStatus,
        waits_for: &[Uuid],
    ) -> Result<Task, DatabaseError> {
        let task = sqlx::query_as::<_, Task>(
            r#"
            INSERT INTO ci.tasks (id, repository_id, name, command, build_id, s2_uri, status, waits_for)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, repository_id, build_id, s2_uri, name, command, status, waits_for, runner_id, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(repository_id)
        .bind(name)
        .bind(command)
        .bind(build_id)
        .bind(s2_uri)
        .bind(status)
        .bind(waits_for)
        .fetch_one(&self.pool)
        .await?;

        Ok(task)
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<Task>, DatabaseError> {
        let task = sqlx::query_as::<_, Task>(
            r#"
            SELECT id, repository_id, build_id, s2_uri, name, command, status, waits_for, runner_id, created_at, updated_at
            FROM ci.tasks WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(task)
    }

    async fn list_by_build_id(&self, build_id: Uuid) -> Result<Vec<Task>, DatabaseError> {
        let tasks = sqlx::query_as::<_, Task>(
            r#"
            SELECT id, repository_id, build_id, s2_uri, name, command, status, waits_for, runner_id, created_at, updated_at
            FROM ci.tasks WHERE build_id = $1
            ORDER BY created_at ASC
            "#,
        )
        .bind(build_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(tasks)
    }

    async fn update_task(&self, id: Uuid, status: TaskStatus) -> Result<Task, DatabaseError> {
        let task = sqlx::query_as::<_, Task>(
            r#"
            UPDATE ci.tasks SET status = $1, updated_at = NOW()
            WHERE id = $2
            RETURNING id, repository_id, build_id, s2_uri, name, command, status, waits_for, runner_id, created_at, updated_at
            "#,
        )
        .bind(status)
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(task)
    }

    async fn claim_task(
        &self,
        runner_id: Uuid,
        repository_ids: &[Uuid],
    ) -> Result<Option<Task>, DatabaseError> {
        let task = sqlx::query_as::<_, Task>(
            r#"
            UPDATE ci.tasks SET status = 'assigned', runner_id = $1, updated_at = NOW()
            WHERE id = (
                SELECT id FROM ci.tasks
                WHERE status = 'pending'
                  AND repository_id = ANY($2)
                ORDER BY created_at ASC
                LIMIT 1
                FOR UPDATE SKIP LOCKED
            )
            RETURNING id, repository_id, build_id, s2_uri, name, command, status, waits_for, runner_id, created_at, updated_at
            "#,
        )
        .bind(runner_id)
        .bind(repository_ids)
        .fetch_optional(&self.pool)
        .await?;

        Ok(task)
    }

    async fn unblock_tasks(&self, build_id: Uuid) -> Result<Vec<Task>, DatabaseError> {
        let tasks = sqlx::query_as::<_, Task>(
            r#"
            UPDATE ci.tasks
            SET status = 'pending', updated_at = NOW()
            WHERE build_id = $1
              AND status = 'blocked'
              AND NOT EXISTS (
                SELECT 1 FROM unnest(waits_for) AS dep_id
                JOIN ci.tasks t2 ON t2.id = dep_id
                WHERE t2.status != 'success'
              )
            RETURNING id, repository_id, build_id, s2_uri, name, command, status, waits_for, runner_id, created_at, updated_at
            "#,
        )
        .bind(build_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(tasks)
    }
}
