use std::time::Duration;

use async_trait::async_trait;
use tokio::time::{Instant, sleep};
use uuid::Uuid;

use crate::{
    dto::{TaskResponse, UpdateTaskRequest},
    error::TaskError,
    repository::{TaskRepository, TaskRepositoryImpl},
};

#[async_trait]
pub trait TaskService: Send + Sync + 'static {
    async fn get_task(&self, id: Uuid) -> Result<TaskResponse, TaskError>;
    async fn update_task(&self, req: UpdateTaskRequest) -> Result<TaskResponse, TaskError>;
    async fn poll_task(&self) -> Result<Option<TaskResponse>, TaskError>;
}

#[derive(Debug, Clone)]
pub struct TaskServiceImpl<R>
where
    R: TaskRepository,
{
    task_repo: R,
}

impl TaskServiceImpl<TaskRepositoryImpl> {
    pub fn new(task_repo: TaskRepositoryImpl) -> Self {
        Self { task_repo }
    }
}

#[async_trait]
impl<R> TaskService for TaskServiceImpl<R>
where
    R: TaskRepository,
{
    async fn get_task(&self, id: Uuid) -> Result<TaskResponse, TaskError> {
        let task = self.task_repo.get_by_id(id).await.map_err(|e| match e {
            sqlx::Error::RowNotFound => TaskError::NotFound(id.to_string()),
            e => TaskError::DatabaseError(e),
        })?;

        Ok(task.into())
    }

    async fn update_task(&self, req: UpdateTaskRequest) -> Result<TaskResponse, TaskError> {
        let task = self
            .task_repo
            .update_task(req.id, req.status)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => TaskError::NotFound(req.id.to_string()),
                e => TaskError::DatabaseError(e),
            })?;

        Ok(task.into())
    }

    async fn poll_task(&self) -> Result<Option<TaskResponse>, TaskError> {
        let deadline = Instant::now() + Duration::from_secs(60);

        loop {
            let task = self
                .task_repo
                .claim_task()
                .await
                .map_err(TaskError::DatabaseError)?;

            if let Some(task) = task {
                return Ok(Some(task.into()));
            }
            if Instant::now() >= deadline {
                return Ok(None);
            }

            sleep(Duration::from_secs(1)).await;
        }
    }
}
