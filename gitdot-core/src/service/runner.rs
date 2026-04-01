use async_trait::async_trait;

use crate::{
    client::{TokenClient, TokenClientImpl},
    dto::{
        CreateRunnerRequest, CreateRunnerResponse, CreateRunnerTokenRequest,
        CreateRunnerTokenResponse, DeleteRunnerRequest, GetRunnerRequest, GetRunnerResponse,
        ListRunnersRequest, ListRunnersResponse, VerifyRunnerRequest,
    },
    error::{NotFoundError, RunnerError},
    model::{RunnerOwnerType, TokenType},
    repository::{
        OrganizationRepository, OrganizationRepositoryImpl, RunnerRepository, RunnerRepositoryImpl,
        TokenRepository, TokenRepositoryImpl,
    },
};

#[async_trait]
pub trait RunnerService: Send + Sync + 'static {
    async fn create_runner(
        &self,
        request: CreateRunnerRequest,
    ) -> Result<CreateRunnerResponse, RunnerError>;

    async fn verify_runner(&self, request: VerifyRunnerRequest) -> Result<(), RunnerError>;

    async fn get_runner(&self, request: GetRunnerRequest)
    -> Result<GetRunnerResponse, RunnerError>;

    async fn delete_runner(&self, request: DeleteRunnerRequest) -> Result<(), RunnerError>;

    async fn refresh_runner_token(
        &self,
        request: CreateRunnerTokenRequest,
    ) -> Result<CreateRunnerTokenResponse, RunnerError>;

    async fn list_runners(
        &self,
        request: ListRunnersRequest,
    ) -> Result<ListRunnersResponse, RunnerError>;
}

#[derive(Debug, Clone)]
pub struct RunnerServiceImpl<R, O, T, TC>
where
    R: RunnerRepository,
    O: OrganizationRepository,
    T: TokenRepository,
    TC: TokenClient,
{
    runner_repo: R,
    org_repo: O,
    token_repo: T,
    token_client: TC,
}

impl
    RunnerServiceImpl<
        RunnerRepositoryImpl,
        OrganizationRepositoryImpl,
        TokenRepositoryImpl,
        TokenClientImpl,
    >
{
    pub fn new(
        runner_repo: RunnerRepositoryImpl,
        org_repo: OrganizationRepositoryImpl,
        token_repo: TokenRepositoryImpl,
        token_client: TokenClientImpl,
    ) -> Self {
        Self {
            runner_repo,
            org_repo,
            token_repo,
            token_client,
        }
    }
}

#[crate::instrument_all]
#[async_trait]
impl<R, O, T, TC> RunnerService for RunnerServiceImpl<R, O, T, TC>
where
    R: RunnerRepository,
    O: OrganizationRepository,
    T: TokenRepository,
    TC: TokenClient,
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
                    .ok_or_else(|| NotFoundError::new("owner", request.owner_name.as_ref()))?;
                org.id
            }
        };

        let runner = self
            .runner_repo
            .create(
                request.name.as_ref(),
                owner_id,
                request.owner_name.as_ref(),
                &request.owner_type,
            )
            .await?;

        Ok(runner.into())
    }

    async fn verify_runner(&self, request: VerifyRunnerRequest) -> Result<(), RunnerError> {
        self.runner_repo
            .touch(request.runner_id)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => {
                    RunnerError::NotFound(NotFoundError::new("runner", request.runner_id))
                }
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
            .get(request.owner_name.as_ref(), request.name.as_ref())
            .await
            .map_err(RunnerError::DatabaseError)?
            .ok_or_else(|| NotFoundError::new("runner", request.name.as_ref()))?;

        Ok(runner.into())
    }

    async fn delete_runner(&self, request: DeleteRunnerRequest) -> Result<(), RunnerError> {
        let runner = self
            .runner_repo
            .get(request.owner_name.as_ref(), request.name.as_ref())
            .await
            .map_err(RunnerError::DatabaseError)?
            .ok_or_else(|| NotFoundError::new("runner", request.name.as_ref()))?;

        self.runner_repo
            .delete(runner.id)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => {
                    RunnerError::NotFound(NotFoundError::new("runner", request.name.as_ref()))
                }
                e => RunnerError::DatabaseError(e),
            })?;

        Ok(())
    }

    async fn list_runners(
        &self,
        request: ListRunnersRequest,
    ) -> Result<ListRunnersResponse, RunnerError> {
        let runners = self
            .runner_repo
            .list_by_owner(request.owner_name.as_ref())
            .await
            .map_err(RunnerError::DatabaseError)?;

        Ok(runners.into_iter().map(Into::into).collect())
    }

    async fn refresh_runner_token(
        &self,
        request: CreateRunnerTokenRequest,
    ) -> Result<CreateRunnerTokenResponse, RunnerError> {
        let runner = self
            .runner_repo
            .get(request.owner_name.as_ref(), request.runner_name.as_ref())
            .await
            .map_err(RunnerError::DatabaseError)?
            .ok_or_else(|| NotFoundError::new("runner", request.runner_name.as_ref()))?;

        self.token_repo.delete_token_by_principal(runner.id).await?;

        let (raw_token, token_hash) = self.token_client.generate_access_token(&TokenType::Runner);

        let client_id = format!(
            "gitdot-runner/{}/{}",
            request.owner_name.as_ref(),
            request.runner_name.as_ref()
        );

        self.token_repo
            .create_token(runner.id, &client_id, &token_hash, TokenType::Runner)
            .await?;

        Ok(CreateRunnerTokenResponse { token: raw_token })
    }
}
