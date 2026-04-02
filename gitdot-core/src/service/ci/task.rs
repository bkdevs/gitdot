use std::time::Duration;

use async_trait::async_trait;
use tokio::time::{Instant, sleep};
use uuid::Uuid;

use crate::{
    dto::{TaskResponse, UpdateTaskRequest},
    error::{NotFoundError, NotFoundExt, TaskError},
    model::TaskStatus,
    repository::{
        RepositoryRepository, RepositoryRepositoryImpl, RunnerRepository, RunnerRepositoryImpl,
        TaskRepository, TaskRepositoryImpl,
    },
};

#[async_trait]
pub trait TaskService: Send + Sync + 'static {
    async fn get_task(&self, id: Uuid) -> Result<Option<TaskResponse>, TaskError>;

    async fn update_task(&self, req: UpdateTaskRequest) -> Result<TaskResponse, TaskError>;

    async fn poll_task(&self, runner_id: Uuid) -> Result<Option<TaskResponse>, TaskError>;
}

#[derive(Debug, Clone)]
pub struct TaskServiceImpl<T, R, S>
where
    T: TaskRepository,
    R: RunnerRepository,
    S: RepositoryRepository,
{
    task_repo: T,
    runner_repo: R,
    repository_repo: S,
}

impl TaskServiceImpl<TaskRepositoryImpl, RunnerRepositoryImpl, RepositoryRepositoryImpl> {
    pub fn new(
        task_repo: TaskRepositoryImpl,
        runner_repo: RunnerRepositoryImpl,
        repository_repo: RepositoryRepositoryImpl,
    ) -> Self {
        Self {
            task_repo,
            runner_repo,
            repository_repo,
        }
    }
}

#[crate::instrument_all]
#[async_trait]
impl<T, R, S> TaskService for TaskServiceImpl<T, R, S>
where
    T: TaskRepository,
    R: RunnerRepository,
    S: RepositoryRepository,
{
    async fn get_task(&self, id: Uuid) -> Result<Option<TaskResponse>, TaskError> {
        let task = self.task_repo.get_by_id(id).await?;

        Ok(task.map(Into::into))
    }

    async fn update_task(&self, req: UpdateTaskRequest) -> Result<TaskResponse, TaskError> {
        let task = self
            .task_repo
            .update_task(req.id, req.status)
            .await
            .or_not_found::<TaskError>("task", req.id)?;

        if task.status == TaskStatus::Success {
            self.task_repo.unblock_tasks(task.build_id).await?;
        }

        Ok(task.into())
    }

    async fn poll_task(&self, runner_id: Uuid) -> Result<Option<TaskResponse>, TaskError> {
        self.runner_repo.touch(runner_id).await?;

        let runner = self
            .runner_repo
            .get_by_id(runner_id)
            .await?
            .ok_or_else(|| NotFoundError::new("runner", runner_id))?;

        let repos = self
            .repository_repo
            .list_by_owner(&runner.owner_name)
            .await?;

        let repository_ids: Vec<Uuid> = repos.iter().map(|r| r.id).collect();

        let deadline = Instant::now() + Duration::from_secs(60);

        loop {
            let task = self
                .task_repo
                .claim_task(runner_id, &repository_ids)
                .await?;

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
