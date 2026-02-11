use async_trait::async_trait;

use crate::dto::{TaskResponse, UpdateTaskRequest};
use crate::error::TaskError;
use crate::repository::{TaskRepository, TaskRepositoryImpl};

#[async_trait]
pub trait TaskService: Send + Sync + 'static {
    async fn update_task(&self, request: UpdateTaskRequest) -> Result<TaskResponse, TaskError>;
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
    async fn update_task(&self, request: UpdateTaskRequest) -> Result<TaskResponse, TaskError> {
        let task = self
            .task_repo
            .update(request.id, &request.status)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => TaskError::NotFound(request.id.to_string()),
                e => TaskError::DatabaseError(e),
            })?;

        Ok(task.into())
    }
}
