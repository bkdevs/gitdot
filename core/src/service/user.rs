use async_trait::async_trait;

use crate::dto::{
    GetCurrentUserRequest, GetUserRequest, ListUserRepositoriesRequest, RepositoryResponse,
    UserResponse,
};
use crate::error::UserError;
use crate::repository::{
    RepositoryRepository, RepositoryRepositoryImpl, UserRepository, UserRepositoryImpl,
};

#[async_trait]
pub trait UserService: Send + Sync + 'static {
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
pub struct UserServiceImpl<U, R>
where
    U: UserRepository,
    R: RepositoryRepository,
{
    user_repo: U,
    repo_repo: R,
}

impl UserServiceImpl<UserRepositoryImpl, RepositoryRepositoryImpl> {
    pub fn new(user_repo: UserRepositoryImpl, repo_repo: RepositoryRepositoryImpl) -> Self {
        Self {
            user_repo,
            repo_repo,
        }
    }
}

#[async_trait]
impl<U, R> UserService for UserServiceImpl<U, R>
where
    U: UserRepository,
    R: RepositoryRepository,
{
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
