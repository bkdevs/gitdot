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

        // 2. Parse and validate
        let ci_config = CiConfig::new(&file.content).map_err(BuildError::InvalidConfig)?;

        // 3. Find the BuildConfig matching trigger
        let build_config = ci_config
            .builds
            .iter()
            .find(|b| b.trigger == request.trigger)
            .ok_or(BuildError::NoMatchingBuildConfig)?;

        let trigger_str: String = request.trigger.clone().into();

        // 4. Create the Build with empty task_dependencies initially
        let build = self
            .build_repo
            .create(
                owner,
                repo,
                &trigger_str,
                &request.commit_sha,
                &HashMap::new(),
            )
            .await?;

        // 5. Resolve TaskConfigs for this build's task names
        let task_names: &[String] = &build_config.tasks;
        let task_configs: Vec<_> = ci_config
            .tasks
            .iter()
            .filter(|t| task_names.contains(&t.name))
            .collect();

        // 6. Create each Task as Blocked
        let mut created_tasks: Vec<(String, crate::model::Task)> = Vec::new();
        for task_config in &task_configs {
            let task = self
                .task_repo
                .create(
                    owner,
                    repo,
                    &task_config.command,
                    build.id,
                    TaskStatus::Blocked,
                )
                .await?;
            created_tasks.push((task_config.name.clone(), task));
        }

        // 7. Build name → task_id map
        let name_to_id: HashMap<String, Uuid> = created_tasks
            .iter()
            .map(|(name, task)| (name.clone(), task.id))
            .collect();

        // 8. Compute task_dependencies: HashMap<Uuid, Vec<Uuid>>
        let mut task_dependencies: HashMap<Uuid, Vec<Uuid>> = HashMap::new();
        for (name, task) in &created_tasks {
            let task_config = task_configs.iter().find(|t| &t.name == name).unwrap();
            if let Some(runs_after) = &task_config.runs_after {
                let deps: Vec<Uuid> = runs_after
                    .iter()
                    .filter_map(|dep_name| name_to_id.get(dep_name).copied())
                    .collect();
                if !deps.is_empty() {
                    task_dependencies.insert(task.id, deps);
                }
            }
        }

        // 9. Update tasks with no runs_after to Pending
        let mut task_responses: Vec<TaskResponse> = Vec::new();
        for (_name, task) in &created_tasks {
            let has_deps = task_dependencies.contains_key(&task.id);
            let updated_task = if !has_deps {
                self.task_repo
                    .update_task(task.id, TaskStatus::Pending)
                    .await?
            } else {
                task.clone()
            };
            task_responses.push(updated_task.into());
        }

        // 10. Update build's task_dependencies
        let updated_build = self
            .build_repo
            .update_task_dependencies(build.id, &task_dependencies)
            .await?;

        // 11. Return BuildResponse
        Ok(BuildResponse {
            id: updated_build.id,
            repo_owner: updated_build.repo_owner,
            repo_name: updated_build.repo_name,
            trigger: updated_build.trigger,
            commit_sha: updated_build.commit_sha,
            task_dependencies: updated_build.task_dependencies.0,
            tasks: task_responses,
            created_at: updated_build.created_at,
            updated_at: updated_build.updated_at,
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
