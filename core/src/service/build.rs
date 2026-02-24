use async_trait::async_trait;

use crate::{
    client::{Git2Client, GitClient},
    dto::{BuildConfig, BuildResponse, CreateBuildRequest, GetBuildConfigRequest},
    error::{BuildError, GitError},
    repository::{BuildRepository, BuildRepositoryImpl},
};

#[async_trait]
pub trait BuildService: Send + Sync + 'static {
    async fn create_build(&self, request: CreateBuildRequest) -> Result<BuildResponse, BuildError>;

    async fn get_build_config(
        &self,
        request: GetBuildConfigRequest,
    ) -> Result<BuildConfig, BuildError>;
}

#[derive(Debug, Clone)]
pub struct BuildServiceImpl<R>
where
    R: BuildRepository,
{
    build_repo: R,
    git_client: Git2Client,
}

impl BuildServiceImpl<BuildRepositoryImpl> {
    pub fn new(build_repo: BuildRepositoryImpl, git_client: Git2Client) -> Self {
        Self {
            build_repo,
            git_client,
        }
    }
}

#[async_trait]
impl<R> BuildService for BuildServiceImpl<R>
where
    R: BuildRepository,
{
    async fn create_build(&self, request: CreateBuildRequest) -> Result<BuildResponse, BuildError> {
        let build = self
            .build_repo
            .create(
                request.repo_owner.as_ref(),
                request.repo_name.as_ref(),
                &request.task_dependencies,
            )
            .await?;

        Ok(build.into())
    }

    async fn get_build_config(
        &self,
        request: GetBuildConfigRequest,
    ) -> Result<BuildConfig, BuildError> {
        let file = self
            .git_client
            .get_repo_file(
                request.owner_name.as_ref(),
                request.repo_name.as_ref(),
                &request.ref_name,
                ".gitdot-ci.toml",
            )
            .await
            .map_err(|e: GitError| match e {
                GitError::Git2Error(ref git2_err)
                    if git2_err.code() == git2::ErrorCode::NotFound =>
                {
                    BuildError::ConfigNotFound(request.ref_name.clone())
                }
                other => BuildError::GitError(other),
            })?;

        toml::from_str(&file.content).map_err(BuildError::ParseError)
    }
}
