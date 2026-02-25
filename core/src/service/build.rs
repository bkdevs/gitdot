use std::collections::HashMap;

use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    client::{Git2Client, GitClient},
    dto::{BuildResponse, CiConfig, CreateBuildRequest, ListBuildsRequest, TaskResponse},
    error::{BuildError, GitError},
    model::TaskStatus,
    repository::{BuildRepository, BuildRepositoryImpl, TaskRepository, TaskRepositoryImpl},
};

#[async_trait]
pub trait BuildService: Send + Sync + 'static {
    async fn create_build(&self, request: CreateBuildRequest) -> Result<BuildResponse, BuildError>;

    async fn list_builds(
        &self,
        request: ListBuildsRequest,
    ) -> Result<Vec<BuildResponse>, BuildError>;
}

#[derive(Debug, Clone)]
pub struct BuildServiceImpl<B, T>
where
    B: BuildRepository,
    T: TaskRepository,
{
    build_repo: B,
    task_repo: T,
    git_client: Git2Client,
}

impl BuildServiceImpl<BuildRepositoryImpl, TaskRepositoryImpl> {
    pub fn new(
        build_repo: BuildRepositoryImpl,
        task_repo: TaskRepositoryImpl,
        git_client: Git2Client,
    ) -> Self {
        Self {
            build_repo,
            task_repo,
            git_client,
        }
    }
}

#[async_trait]
impl<B, T> BuildService for BuildServiceImpl<B, T>
where
    B: BuildRepository,
    T: TaskRepository,
{
    async fn create_build(&self, request: CreateBuildRequest) -> Result<BuildResponse, BuildError> {
        let owner = request.repo_owner.as_ref();
        let repo = request.repo_name.as_ref();

        let file = self
            .git_client
            .get_repo_file(owner, repo, &request.commit_sha, ".gitdot-ci.toml")
            .await
            .map_err(|e: GitError| match e {
                GitError::Git2Error(ref git2_err)
                    if git2_err.code() == git2::ErrorCode::NotFound =>
                {
                    BuildError::ConfigNotFound(request.commit_sha.clone())
                }
                other => BuildError::GitError(other),
            })?;

        let ci_config = CiConfig::new(&file.content).map_err(BuildError::InvalidConfig)?;
        let build_config = ci_config.get_build_config(&request.trigger)?;
        let task_configs = ci_config.get_task_configs(build_config);
        let trigger_str = match &request.trigger {
            crate::dto::BuildTrigger::PullRequest => "pull_request",
            crate::dto::BuildTrigger::PushToMain => "push_to_main",
        };

        let build = self
            .build_repo
            .create(owner, repo, trigger_str, &request.commit_sha)
            .await?;

        // Pre-generate UUIDs for all tasks so dependencies can reference each other by ID
        let mut name_to_id: HashMap<String, Uuid> = HashMap::new();
        for task_config in &task_configs {
            name_to_id.insert(task_config.name.clone(), Uuid::new_v4());
        }

        let mut task_responses: Vec<TaskResponse> = Vec::new();
        for task_config in &task_configs {
            let id = name_to_id[&task_config.name];
            let waits_for: Vec<Uuid> = task_config
                .waits_for
                .as_deref()
                .unwrap_or(&[])
                .iter()
                .filter_map(|dep_name| name_to_id.get(dep_name).copied())
                .collect();
            let status = if waits_for.is_empty() {
                TaskStatus::Pending
            } else {
                TaskStatus::Blocked
            };

            let task = self
                .task_repo
                .create(
                    id,
                    owner,
                    repo,
                    &task_config.name,
                    &task_config.command,
                    build.id,
                    status,
                    &waits_for,
                )
                .await?;
            task_responses.push(task.into());
        }

        Ok(BuildResponse {
            id: build.id,
            repo_owner: build.repo_owner,
            repo_name: build.repo_name,
            trigger: build.trigger,
            commit_sha: build.commit_sha,
            tasks: task_responses,
            created_at: build.created_at,
            updated_at: build.updated_at,
        })
    }

    async fn list_builds(
        &self,
        request: ListBuildsRequest,
    ) -> Result<Vec<BuildResponse>, BuildError> {
        let builds = self
            .build_repo
            .list_by_repo(request.repo_owner.as_ref(), request.repo_name.as_ref())
            .await
            .map_err(BuildError::DatabaseError)?;

        Ok(builds.into_iter().map(Into::into).collect())
    }
}
