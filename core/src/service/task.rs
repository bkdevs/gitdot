use std::{collections::HashSet, time::Duration};

use async_trait::async_trait;
use tokio::time::{Instant, sleep};
use uuid::Uuid;

use crate::{
    dto::{CiConfig, CreateTaskRequest, ListTasksRequest, TaskResponse, UpdateTaskRequest},
    error::TaskError,
    model::{Task, TaskStatus},
    repository::{BuildRepository, BuildRepositoryImpl, TaskRepository, TaskRepositoryImpl},
};

#[async_trait]
pub trait TaskService: Send + Sync + 'static {
    async fn get_task(&self, id: Uuid) -> Result<TaskResponse, TaskError>;
    async fn list_tasks(&self, req: ListTasksRequest) -> Result<Vec<TaskResponse>, TaskError>;
    async fn create_task(&self, req: CreateTaskRequest) -> Result<TaskResponse, TaskError>;
    async fn update_task(&self, req: UpdateTaskRequest) -> Result<TaskResponse, TaskError>;
    async fn poll_task(&self) -> Result<Option<TaskResponse>, TaskError>;
}

#[derive(Debug, Clone)]
pub struct TaskServiceImpl<R, B>
where
    R: TaskRepository,
    B: BuildRepository,
{
    task_repo: R,
    build_repo: B,
}

impl TaskServiceImpl<TaskRepositoryImpl, BuildRepositoryImpl> {
    pub fn new(task_repo: TaskRepositoryImpl, build_repo: BuildRepositoryImpl) -> Self {
        Self {
            task_repo,
            build_repo,
        }
    }
}

impl<R, B> TaskServiceImpl<R, B>
where
    R: TaskRepository,
    B: BuildRepository,
{
    async fn create_downstream_tasks(&self, completed_task: &Task) -> Result<(), TaskError> {
        let build = self
            .build_repo
            .get_by_id(completed_task.build_id)
            .await
            .map_err(TaskError::DatabaseError)?;

        let ci_config = CiConfig::new(&build.build_config)?;

        let build_config = ci_config
            .builds
            .iter()
            .find(|b| {
                let trigger_str: String = b.trigger.clone().into();
                trigger_str == build.trigger
            })
            .ok_or(TaskError::NoBuildConfig)?;

        let existing_tasks = self
            .task_repo
            .list_by_build_id(build.id)
            .await
            .map_err(TaskError::DatabaseError)?;

        let created_names: HashSet<&str> = existing_tasks.iter().map(|t| t.name.as_str()).collect();

        let succeeded_names: HashSet<&str> = existing_tasks
            .iter()
            .filter(|t| t.status == TaskStatus::Success)
            .map(|t| t.name.as_str())
            .collect();

        for task_name in &build_config.tasks {
            if created_names.contains(task_name.as_str()) {
                continue;
            }

            let task_config = match ci_config.tasks.iter().find(|t| &t.name == task_name) {
                Some(tc) => tc,
                None => continue,
            };

            let all_deps_succeeded = task_config
                .runs_after
                .as_deref()
                .unwrap_or(&[])
                .iter()
                .all(|dep| succeeded_names.contains(dep.as_str()));

            if all_deps_succeeded {
                self.task_repo
                    .create(
                        &build.repo_owner,
                        &build.repo_name,
                        task_name,
                        &task_config.command,
                        build.id,
                        TaskStatus::Pending,
                    )
                    .await
                    .map_err(TaskError::DatabaseError)?;
            }
        }

        Ok(())
    }
}

#[async_trait]
impl<R, B> TaskService for TaskServiceImpl<R, B>
where
    R: TaskRepository,
    B: BuildRepository,
{
    async fn get_task(&self, id: Uuid) -> Result<TaskResponse, TaskError> {
        let task = self.task_repo.get_by_id(id).await.map_err(|e| match e {
            sqlx::Error::RowNotFound => TaskError::NotFound(id.to_string()),
            e => TaskError::DatabaseError(e),
        })?;

        Ok(task.into())
    }

    async fn list_tasks(&self, req: ListTasksRequest) -> Result<Vec<TaskResponse>, TaskError> {
        let tasks = self
            .task_repo
            .list_by_repo(req.repo_owner.as_ref(), req.repo_name.as_ref())
            .await
            .map_err(TaskError::DatabaseError)?;

        Ok(tasks.into_iter().map(Into::into).collect())
    }

    async fn create_task(&self, req: CreateTaskRequest) -> Result<TaskResponse, TaskError> {
        let task = self
            .task_repo
            .create(
                req.repo_owner.as_ref(),
                req.repo_name.as_ref(),
                &req.name,
                &req.script,
                req.build_id,
                TaskStatus::Pending,
            )
            .await
            .map_err(TaskError::DatabaseError)?;

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
            self.create_downstream_tasks(&task).await?;
        }

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
