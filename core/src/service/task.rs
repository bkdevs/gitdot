use async_trait::async_trait;
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
        let task = self
            .task_repo
            .get_by_id(id)
            .await
            .map_err(|e| match e {
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
}
