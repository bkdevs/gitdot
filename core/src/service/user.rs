use async_trait::async_trait;

use crate::client::{SupabaseClient, SupabaseClientImpl};
use crate::dto::{
    CreateUserRequest, GetCurrentUserRequest, GetUserRequest, ListUserRepositoriesRequest,
    RepositoryResponse, UserResponse,
};
use crate::error::UserError;
use crate::repository::{
    RepositoryRepository, RepositoryRepositoryImpl, UserRepository, UserRepositoryImpl,
};
use crate::util::auth::is_reserved_name;

#[async_trait]
pub trait UserService: Send + Sync + 'static {
    async fn create_user(&self, request: CreateUserRequest) -> Result<(), UserError>;

    async fn get_current_user(
        &self,
        request: GetCurrentUserRequest,
    ) -> Result<UserResponse, UserError>;

    async fn get_user(&self, request: GetUserRequest) -> Result<UserResponse, UserError>;

    async fn list_repositories(
        &self,
        request: ListUserRepositoriesRequest,
    ) -> Result<Vec<RepositoryResponse>, UserError>;
}

#[derive(Debug, Clone)]
pub struct UserServiceImpl<U, R, S>
where
    U: UserRepository,
    R: RepositoryRepository,
    S: SupabaseClient,
{
    user_repo: U,
    repo_repo: R,
    supabase_client: S,
}

impl UserServiceImpl<UserRepositoryImpl, RepositoryRepositoryImpl, SupabaseClientImpl> {
    pub fn new(
        user_repo: UserRepositoryImpl,
        repo_repo: RepositoryRepositoryImpl,
        supabase_client: SupabaseClientImpl,
    ) -> Self {
        Self {
            user_repo,
            repo_repo,
            supabase_client,
        }
    }
}

#[async_trait]
impl<U, R, S> UserService for UserServiceImpl<U, R, S>
where
    U: UserRepository,
    R: RepositoryRepository,
    S: SupabaseClient,
{
    async fn create_user(&self, request: CreateUserRequest) -> Result<(), UserError> {
        let name = request.name.to_string();

        if is_reserved_name(&name) {
            return Err(UserError::ReservedName(name));
        }

        if self.user_repo.is_name_taken(&name).await? {
            return Err(UserError::NameTaken(name));
        }

        // Once Supabase user is created, the Postgres trigger will then
        // create the corresponding user row in users table.
        self.supabase_client
            .create_user(&name, &request.email, &request.password)
            .await?;

        Ok(())
    }

    async fn get_current_user(
        &self,
        request: GetCurrentUserRequest,
    ) -> Result<UserResponse, UserError> {
        let user = self
            .user_repo
            .get_by_id(request.user_id)
            .await?
            .ok_or_else(|| UserError::NotFound(request.user_id.to_string()))?;
        Ok(user.into())
    }

    async fn get_user(&self, request: GetUserRequest) -> Result<UserResponse, UserError> {
        let user_name = request.user_name.to_string();
        let user = self
            .user_repo
            .get(&user_name)
            .await?
            .ok_or_else(|| UserError::NotFound(user_name))?;
        Ok(user.into())
    }

    async fn list_repositories(
        &self,
        request: ListUserRepositoriesRequest,
    ) -> Result<Vec<RepositoryResponse>, UserError> {
        let user_name = request.user_name.to_string();
        let user = self
            .user_repo
            .get(&user_name)
            .await?
            .ok_or_else(|| UserError::NotFound(user_name.clone()))?;

        let repositories = self.repo_repo.list_by_owner(&user_name).await?;

        let is_owner = request.viewer_id.map(|id| id == user.id).unwrap_or(false);
        let repositories = if is_owner {
            repositories
        } else {
            repositories.into_iter().filter(|r| r.is_public()).collect()
        };

        Ok(repositories.into_iter().map(|r| r.into()).collect())
    }
}
