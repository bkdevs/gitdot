use async_trait::async_trait;

use crate::{
    dto::{
        CreateRunnerRequest, CreateRunnerResponse, CreateRunnerTokenRequest,
        CreateRunnerTokenResponse, DeleteRunnerRequest, GetRunnerRequest, GetRunnerResponse,
        VerifyRunnerRequest,
    },
    error::RunnerError,
    model::{RunnerOwnerType, TokenType},
    repository::{
        OrganizationRepository, OrganizationRepositoryImpl, RunnerRepository, RunnerRepositoryImpl,
        TokenRepository, TokenRepositoryImpl,
    },
    util::token::{generate_access_token, hash_token},
};

#[async_trait]
pub trait RunnerService: Send + Sync + 'static {
    async fn create_runner(
        &self,
        request: CreateRunnerRequest,
    ) -> Result<CreateRunnerResponse, RunnerError>;
    async fn delete_runner(&self, request: DeleteRunnerRequest) -> Result<(), RunnerError>;
    async fn create_runner_token(
        &self,
        request: CreateRunnerTokenRequest,
    ) -> Result<CreateRunnerTokenResponse, RunnerError>;
    async fn verify_runner(&self, request: VerifyRunnerRequest) -> Result<(), RunnerError>;
    async fn get_runner(&self, request: GetRunnerRequest)
    -> Result<GetRunnerResponse, RunnerError>;
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

    async fn create_runner_token(
        &self,
        request: CreateRunnerTokenRequest,
    ) -> Result<CreateRunnerTokenResponse, RunnerError> {
        self.token_repo
            .delete_runner_token(request.runner_id)
            .await?;

        let raw_token = generate_access_token(&TokenType::Runner);
        let token_hash = hash_token(&raw_token);

        self.token_repo
            .create_runner_token(request.runner_id, &token_hash)
            .await?;

        Ok(CreateRunnerTokenResponse { token: raw_token })
    }

    async fn verify_runner(&self, request: VerifyRunnerRequest) -> Result<(), RunnerError> {
        self.runner_repo
            .touch(request.runner_id)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => RunnerError::NotFound(request.runner_id.to_string()),
                e => RunnerError::DatabaseError(e),
            })?;

        Ok(())
    }

    async fn get_runner(
        &self,
        request: GetRunnerRequest,
    ) -> Result<GetRunnerResponse, RunnerError> {
        let runner = self
            .runner_repo
            .get_by_name(
                request.owner_name.as_ref(),
                &request.owner_type,
                request.name.as_ref(),
            )
            .await?
            .ok_or_else(|| RunnerError::NotFound(request.name.to_string()))?;

        Ok(runner.into())
    }
}
