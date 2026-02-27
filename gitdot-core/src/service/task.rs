use std::time::Duration;

use async_trait::async_trait;
use tokio::time::{Instant, sleep};
use uuid::Uuid;

use crate::{
    dto::{TaskResponse, UpdateTaskRequest},
    error::TaskError,
    model::TaskStatus,
    repository::{RunnerRepository, RunnerRepositoryImpl, TaskRepository, TaskRepositoryImpl},
};

#[async_trait]
pub trait TaskService: Send + Sync + 'static {
    async fn get_task(&self, id: Uuid) -> Result<TaskResponse, TaskError>;

    async fn update_task(&self, req: UpdateTaskRequest) -> Result<TaskResponse, TaskError>;

    async fn poll_task(&self, runner_id: Uuid) -> Result<Option<TaskResponse>, TaskError>;
}

#[derive(Debug, Clone)]
pub struct TaskServiceImpl<T, R>
where
    T: TaskRepository,
    R: RunnerRepository,
{
    task_repo: T,
    runner_repo: R,
}

impl TaskServiceImpl<TaskRepositoryImpl, RunnerRepositoryImpl> {
    pub fn new(task_repo: TaskRepositoryImpl, runner_repo: RunnerRepositoryImpl) -> Self {
        Self {
            task_repo,
            runner_repo,
        }
    }
}

#[async_trait]
impl<T, R> TaskService for TaskServiceImpl<T, R>
where
    T: TaskRepository,
    R: RunnerRepository,
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

        if task.status == TaskStatus::Success {
            self.task_repo
                .unblock_tasks(task.build_id)
                .await
                .map_err(TaskError::DatabaseError)?;
        }

        Ok(task.into())
    }

    async fn poll_task(&self, runner_id: Uuid) -> Result<Option<TaskResponse>, TaskError> {
        self.runner_repo
            .touch(runner_id)
            .await
            .map_err(TaskError::DatabaseError)?;

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
