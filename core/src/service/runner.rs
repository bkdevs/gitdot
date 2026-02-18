use async_trait::async_trait;

use crate::{
    dto::{CreateRunnerRequest, CreateRunnerResponse, DeleteRunnerRequest, RegisterRunnerRequest},
    error::RunnerError,
    model::RunnerOwnerType,
    repository::{
        OrganizationRepository, OrganizationRepositoryImpl, RunnerRepository, RunnerRepositoryImpl,
    },
};

#[async_trait]
pub trait RunnerService: Send + Sync + 'static {
    async fn create_runner(
        &self,
        request: CreateRunnerRequest,
    ) -> Result<CreateRunnerResponse, RunnerError>;
    async fn delete_runner(&self, request: DeleteRunnerRequest) -> Result<(), RunnerError>;
    async fn register_runner(&self, request: RegisterRunnerRequest) -> Result<(), RunnerError>;
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
    ) -> Result<CreateRunnerResponse, RunnerError> {
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

    async fn delete_runner(&self, request: DeleteRunnerRequest) -> Result<(), RunnerError> {
        self.runner_repo
            .delete(request.id)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => RunnerError::NotFound(request.id.to_string()),
                e => RunnerError::DatabaseError(e),
            })?;

        Ok(())
    }

    async fn register_runner(&self, request: RegisterRunnerRequest) -> Result<(), RunnerError> {
        self.runner_repo
            .register(request.id)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => RunnerError::NotFound(request.id.to_string()),
                e => RunnerError::DatabaseError(e),
            })?;

        Ok(())
    }
}
