use async_trait::async_trait;

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

        // 1. Fetch .gitdot-ci.toml from git at commit_sha
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

        let raw_config = file.content;

        // 2. Parse and validate
        let ci_config = CiConfig::new(&raw_config).map_err(BuildError::InvalidConfig)?;

        // 3. Find the BuildConfig matching trigger
        let build_config = ci_config
            .builds
            .iter()
            .find(|b| b.trigger == request.trigger)
            .ok_or(BuildError::NoMatchingBuildConfig)?;

        let trigger_str: String = request.trigger.clone().into();

        // 4. Create the Build, storing raw config
        let build = self
            .build_repo
            .create(owner, repo, &trigger_str, &request.commit_sha, &raw_config)
            .await?;

        // 5. Resolve TaskConfigs for this build's task names
        let task_names: &[String] = &build_config.tasks;
        let task_configs: Vec<_> = ci_config
            .tasks
            .iter()
            .filter(|t| task_names.contains(&t.name))
            .collect();

        // 6. Create only tasks with no runs_after as Pending
        let mut task_responses: Vec<TaskResponse> = Vec::new();
        for task_config in &task_configs {
            let has_deps = task_config
                .runs_after
                .as_deref()
                .map_or(false, |d| !d.is_empty());
            if !has_deps {
                let task = self
                    .task_repo
                    .create(
                        owner,
                        repo,
                        &task_config.name,
                        &task_config.command,
                        build.id,
                        TaskStatus::Pending,
                    )
                    .await?;
                task_responses.push(task.into());
            }
        }

        // 7. Return BuildResponse
        Ok(BuildResponse {
            id: build.id,
            repo_owner: build.repo_owner,
            repo_name: build.repo_name,
            trigger: build.trigger,
            commit_sha: build.commit_sha,
            build_config: build.build_config,
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
