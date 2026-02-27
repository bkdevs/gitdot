use std::collections::HashMap;

use async_trait::async_trait;
use futures::future::try_join_all;
use uuid::Uuid;

use crate::{
    client::{Git2Client, GitClient, S2Client, S2ClientImpl},
    dto::{BuildResponse, CiConfig, CreateBuildRequest, ListBuildsRequest, TaskResponse},
    error::{BuildError, GitError},
    model::{BuildStatus, TaskStatus},
    repository::{
        BuildRepository, BuildRepositoryImpl, RepositoryRepository, RepositoryRepositoryImpl,
        TaskRepository, TaskRepositoryImpl,
    },
};

#[async_trait]
pub trait BuildService: Send + Sync + 'static {
    async fn create_build(&self, request: CreateBuildRequest) -> Result<BuildResponse, BuildError>;

    async fn list_builds(
        &self,
        request: ListBuildsRequest,
    ) -> Result<Vec<BuildResponse>, BuildError>;

    async fn list_build_tasks(&self, build_id: Uuid) -> Result<Vec<TaskResponse>, BuildError>;

    async fn get_build_with_tasks(
        &self,
        owner: &str,
        repo: &str,
        number: i32,
    ) -> Result<(BuildResponse, Vec<TaskResponse>), BuildError>;
}

#[derive(Debug, Clone)]
pub struct BuildServiceImpl<G, S, B, T, R>
where
    G: GitClient,
    S: S2Client,
    B: BuildRepository,
    T: TaskRepository,
    R: RepositoryRepository,
{
    git_client: G,
    s2_client: S,
    build_repo: B,
    task_repo: T,
    repo_repo: R,
}

impl
    BuildServiceImpl<
        Git2Client,
        S2ClientImpl,
        BuildRepositoryImpl,
        TaskRepositoryImpl,
        RepositoryRepositoryImpl,
    >
{
    pub fn new(
        git_client: Git2Client,
        s2_client: S2ClientImpl,
        build_repo: BuildRepositoryImpl,
        task_repo: TaskRepositoryImpl,
        repo_repo: RepositoryRepositoryImpl,
    ) -> Self {
        Self {
            git_client,
            s2_client,
            build_repo,
            task_repo,
            repo_repo,
        }
    }
}

#[async_trait]
impl<G, S, B, T, R> BuildService for BuildServiceImpl<G, S, B, T, R>
where
    G: GitClient,
    S: S2Client,
    B: BuildRepository,
    T: TaskRepository,
    R: RepositoryRepository,
{
    async fn create_build(&self, request: CreateBuildRequest) -> Result<BuildResponse, BuildError> {
        let owner = request.repo_owner.as_ref();
        let repo = request.repo_name.as_ref();

        let repository = self
            .repo_repo
            .get(owner, repo)
            .await
            .map_err(BuildError::DatabaseError)?
            .ok_or_else(|| BuildError::RepositoryNotFound(format!("{owner}/{repo}")))?;

        let commit = self
            .git_client
            .get_repo_commit(owner, repo, &request.commit_sha)
            .await
            .map_err(BuildError::GitError)?;
        let resolved_sha = commit.sha.clone();

        let file = self
            .git_client
            .get_repo_file(owner, repo, &resolved_sha, ".gitdot-ci.toml")
            .await
            .map_err(|e: GitError| match e {
                GitError::Git2Error(ref git2_err)
                    if git2_err.code() == git2::ErrorCode::NotFound =>
                {
                    BuildError::ConfigNotFound(request.commit_sha.clone())
                }
                other => BuildError::GitError(other),
            })?;

        let ci_config =
            CiConfig::new(&file.content).map_err(|e| BuildError::InvalidConfig(e.to_string()))?;
        let build_config = ci_config
            .get_build_config(&request.trigger)
            .map_err(|e| BuildError::InvalidConfig(e.to_string()))?;
        let task_configs = ci_config.get_task_configs(build_config);

        let trigger = crate::model::BuildTrigger::from(request.trigger);
        let build = self
            .build_repo
            .create(repository.id, trigger, &resolved_sha)
            .await?;

        // Pre-generate UUIDs for all tasks so dependencies can reference each other by ID
        let mut name_to_id: HashMap<String, Uuid> = HashMap::new();
        for task_config in &task_configs {
            name_to_id.insert(task_config.name.clone(), Uuid::new_v4());
        }

        let s2_client = &self.s2_client;
        let task_repo = &self.task_repo;
        let repository_id = repository.id;

        let task_futures = task_configs.iter().map(|task_config| {
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

            async move {
                let s2_uri = s2_client
                    .create_stream(owner, repo, id)
                    .await
                    .map_err(BuildError::S2Error)?;

                task_repo
                    .create(
                        id,
                        repository_id,
                        &task_config.name,
                        &task_config.command,
                        build.id,
                        &s2_uri,
                        status,
                        &waits_for,
                    )
                    .await
                    .map_err(BuildError::DatabaseError)
            }
        });

        try_join_all(task_futures).await?;

        let total_tasks = task_configs.len() as i32;
        Ok(BuildResponse {
            id: build.id,
            number: build.number,
            repository_id: build.repository_id,
            trigger: build.trigger,
            commit_sha: build.commit_sha,
            status: BuildStatus::Running,
            total_tasks,
            completed_tasks: 0,
            created_at: build.created_at,
            updated_at: build.created_at,
        })
    }

    async fn list_builds(
        &self,
        request: ListBuildsRequest,
    ) -> Result<Vec<BuildResponse>, BuildError> {
        let owner = request.repo_owner.as_ref();
        let repo = request.repo_name.as_ref();

        let repository = self
            .repo_repo
            .get(owner, repo)
            .await
            .map_err(BuildError::DatabaseError)?
            .ok_or_else(|| BuildError::RepositoryNotFound(format!("{owner}/{repo}")))?;

        let builds = self
            .build_repo
            .list_by_repo(repository.id)
            .await
            .map_err(BuildError::DatabaseError)?;

        Ok(builds.into_iter().map(Into::into).collect())
    }

    async fn list_build_tasks(&self, build_id: Uuid) -> Result<Vec<TaskResponse>, BuildError> {
        let tasks = self
            .task_repo
            .list_by_build_id(build_id)
            .await
            .map_err(BuildError::DatabaseError)?;

        Ok(tasks.into_iter().map(Into::into).collect())
    }

    async fn get_build_with_tasks(
        &self,
        owner: &str,
        repo: &str,
        number: i32,
    ) -> Result<(BuildResponse, Vec<TaskResponse>), BuildError> {
        let repository = self
            .repo_repo
            .get(owner, repo)
            .await
            .map_err(BuildError::DatabaseError)?
            .ok_or_else(|| BuildError::RepositoryNotFound(format!("{owner}/{repo}")))?;

        let build = self
            .build_repo
            .get(repository.id, number)
            .await
            .map_err(BuildError::DatabaseError)?
            .ok_or_else(|| BuildError::NotFound(format!("{owner}/{repo}#{number}")))?;

        let tasks = self
            .task_repo
            .list_by_build_id(build.id)
            .await
            .map_err(BuildError::DatabaseError)?;

        let total_tasks = tasks.len() as i32;
        let completed_tasks = tasks
            .iter()
            .filter(|t| matches!(t.status, TaskStatus::Success))
            .count() as i32;
        let status = if tasks.is_empty() {
            BuildStatus::Running
        } else if tasks.iter().any(|t| t.status == TaskStatus::Failure) {
            BuildStatus::Failure
        } else if tasks.iter().all(|t| t.status == TaskStatus::Success) {
            BuildStatus::Success
        } else {
            BuildStatus::Running
        };

        let effective_updated_at = tasks
            .iter()
            .map(|t| t.updated_at)
            .max()
            .unwrap_or(build.created_at);

        let build_response = BuildResponse {
            id: build.id,
            number: build.number,
            repository_id: build.repository_id,
            trigger: build.trigger,
            commit_sha: build.commit_sha,
            status,
            total_tasks,
            completed_tasks,
            created_at: build.created_at,
            updated_at: effective_updated_at,
        };
        let task_responses = tasks.into_iter().map(Into::into).collect();
        Ok((build_response, task_responses))
    }
}
