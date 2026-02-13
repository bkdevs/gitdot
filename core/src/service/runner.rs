use async_trait::async_trait;

use crate::dto::{
    CreateRunnerRequest, CreateRunnerResponse, DeleteRunnerRequest, RegisterRunnerRequest,
};
use crate::error::RunnerError;
use crate::model::{RunnerOwnerType, TokenType};
use crate::repository::{
    OrganizationRepository, OrganizationRepositoryImpl, RunnerRepository, RunnerRepositoryImpl,
    TokenRepository, TokenRepositoryImpl,
};
use crate::util::token::{generate_access_token, hash_token};

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
pub struct RunnerServiceImpl<R, O, T>
where
    R: RunnerRepository,
    O: OrganizationRepository,
    T: TokenRepository,
{
    runner_repo: R,
    org_repo: O,
    token_repo: T,
}

impl RunnerServiceImpl<RunnerRepositoryImpl, OrganizationRepositoryImpl, TokenRepositoryImpl> {
    pub fn new(
        runner_repo: RunnerRepositoryImpl,
        org_repo: OrganizationRepositoryImpl,
        token_repo: TokenRepositoryImpl,
    ) -> Self {
        Self {
            runner_repo,
            org_repo,
            token_repo,
        }
    }
}

#[async_trait]
impl<R, O, T> RunnerService for RunnerServiceImpl<R, O, T>
where
    R: RunnerRepository,
    O: OrganizationRepository,
    T: TokenRepository,
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

        let token = generate_access_token(&TokenType::Runner);
        let token_hash = hash_token(&token);

        self.token_repo
            .create_runner_token(request.user_id, &token_hash)
            .await
            .map_err(|e| RunnerError::DatabaseError(e))?;

        Ok(CreateRunnerResponse {
            runner: runner.into(),
            token,
        })
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
