use async_trait::async_trait;

use crate::dto::{CreateRunnerRequest, RunnerResponse};
use crate::error::RunnerError;
use crate::model::RunnerOwnerType;
use crate::repository::{
    OrganizationRepository, OrganizationRepositoryImpl, RunnerRepository, RunnerRepositoryImpl,
};

#[async_trait]
pub trait RunnerService: Send + Sync + 'static {
    async fn create_runner(
        &self,
        request: CreateRunnerRequest,
    ) -> Result<RunnerResponse, RunnerError>;
}

#[derive(Debug, Clone)]
pub struct RunnerServiceImpl<R, O>
where
    R: RunnerRepository,
    O: OrganizationRepository,
{
    runner_repo: R,
    org_repo: O,
}

impl RunnerServiceImpl<RunnerRepositoryImpl, OrganizationRepositoryImpl> {
    pub fn new(runner_repo: RunnerRepositoryImpl, org_repo: OrganizationRepositoryImpl) -> Self {
        Self {
            runner_repo,
            org_repo,
        }
    }
}

#[async_trait]
impl<R, O> RunnerService for RunnerServiceImpl<R, O>
where
    R: RunnerRepository,
    O: OrganizationRepository,
{
    async fn create_runner(
        &self,
        request: CreateRunnerRequest,
    ) -> Result<RunnerResponse, RunnerError> {
        let owner_id = match request.owner_type {
            RunnerOwnerType::User => request.user_id,
            RunnerOwnerType::Organization => {
                let org = self
                    .org_repo
                    .get(request.owner_name.as_ref())
                    .await?
                    .ok_or_else(|| RunnerError::OwnerNotFound(request.owner_name.to_string()))?;
                org.id
            }
        };

        let runner = self
            .runner_repo
            .create(request.name.as_ref(), owner_id, &request.owner_type)
            .await?;

        Ok(runner.into())
    }
}
