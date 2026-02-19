use async_trait::async_trait;

use crate::{
    dto::{
        CreateRunnerRequest, CreateRunnerResponse, CreateRunnerTokenRequest,
        CreateRunnerTokenResponse, DeleteRunnerRequest, GetRunnerRequest, GetRunnerResponse,
        ListRunnersRequest, ListRunnersResponse, VerifyRunnerRequest,
    },
    error::RunnerError,
    model::{runner::RunnerOwnerType, token::TokenType},
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
            .get(request.owner_name.as_ref(), request.name.as_ref())
            .await
            .map_err(RunnerError::DatabaseError)?
            .ok_or_else(|| RunnerError::NotFound(request.name.to_string()))?;

        Ok(runner.into())
    }

    async fn delete_runner(&self, request: DeleteRunnerRequest) -> Result<(), RunnerError> {
        let runner = self
            .runner_repo
            .get(request.owner_name.as_ref(), request.name.as_ref())
            .await
            .map_err(RunnerError::DatabaseError)?
            .ok_or_else(|| RunnerError::NotFound(request.name.to_string()))?;

        self.runner_repo
            .delete(runner.id)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => RunnerError::NotFound(request.name.to_string()),
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
            .ok_or_else(|| RunnerError::NotFound(request.runner_name.to_string()))?;

        self.token_repo.delete_token_by_principal(runner.id).await?;

        let raw_token = generate_access_token(&TokenType::Runner);
        let token_hash = hash_token(&raw_token);

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
