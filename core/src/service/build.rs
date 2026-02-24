use async_trait::async_trait;

use crate::{
    dto::{BuildResponse, CreateBuildRequest},
    error::BuildError,
    repository::{BuildRepository, BuildRepositoryImpl},
};

#[async_trait]
pub trait BuildService: Send + Sync + 'static {
    async fn create_build(&self, request: CreateBuildRequest) -> Result<BuildResponse, BuildError>;
}

#[derive(Debug, Clone)]
pub struct BuildServiceImpl<R>
where
    R: BuildRepository,
{
    build_repo: R,
}

impl BuildServiceImpl<BuildRepositoryImpl> {
    pub fn new(build_repo: BuildRepositoryImpl) -> Self {
        Self { build_repo }
    }
}

#[async_trait]
impl<R> BuildService for BuildServiceImpl<R>
where
    R: BuildRepository,
{
    async fn create_build(
        &self,
        request: CreateBuildRequest,
    ) -> Result<BuildResponse, BuildError> {
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
}
