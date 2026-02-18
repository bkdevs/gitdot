use async_trait::async_trait;

use crate::{
    dto::{
        GetCurrentUserRequest, GetUserRequest, HasUserRequest, ListUserOrganizationsRequest,
        ListUserRepositoriesRequest, OrganizationResponse, RepositoryResponse,
        UpdateCurrentUserRequest, UserResponse,
    },
    error::UserError,
    repository::{
        OrganizationRepository, OrganizationRepositoryImpl, RepositoryRepository,
        RepositoryRepositoryImpl, UserRepository, UserRepositoryImpl,
    },
    util::auth::is_reserved_name,
};

#[async_trait]
pub trait UserService: Send + Sync + 'static {
    async fn get_current_user(
        &self,
        request: GetCurrentUserRequest,
    ) -> Result<UserResponse, UserError>;

    async fn update_current_user(
        &self,
        request: UpdateCurrentUserRequest,
    ) -> Result<UserResponse, UserError>;

    async fn has_user(&self, request: HasUserRequest) -> Result<(), UserError>;

    async fn get_user(&self, request: GetUserRequest) -> Result<UserResponse, UserError>;

    async fn list_repositories(
        &self,
        request: ListUserRepositoriesRequest,
    ) -> Result<Vec<RepositoryResponse>, UserError>;

    async fn list_organizations(
        &self,
        request: ListUserOrganizationsRequest,
    ) -> Result<Vec<OrganizationResponse>, UserError>;
}

#[derive(Debug, Clone)]
pub struct UserServiceImpl<U, R, O>
where
    U: UserRepository,
    R: RepositoryRepository,
    O: OrganizationRepository,
{
    user_repo: U,
    repo_repo: R,
    org_repo: O,
}

impl UserServiceImpl<UserRepositoryImpl, RepositoryRepositoryImpl, OrganizationRepositoryImpl> {
    pub fn new(
        user_repo: UserRepositoryImpl,
        repo_repo: RepositoryRepositoryImpl,
        org_repo: OrganizationRepositoryImpl,
    ) -> Self {
        Self {
            user_repo,
            repo_repo,
            org_repo,
        }
    }
}

#[async_trait]
impl<U, R, O> UserService for UserServiceImpl<U, R, O>
where
    U: UserRepository,
    R: RepositoryRepository,
    O: OrganizationRepository,
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

    async fn update_current_user(
        &self,
        request: UpdateCurrentUserRequest,
    ) -> Result<UserResponse, UserError> {
        let name = request.name.to_string();

        if is_reserved_name(&name) {
            return Err(UserError::ReservedName(name));
        }

        if self.user_repo.is_name_taken(&name).await? {
            return Err(UserError::NameTaken(name));
        }

        let user = self.user_repo.update(request.user_id, &name).await?;
        Ok(user.into())
    }

    async fn has_user(&self, request: HasUserRequest) -> Result<(), UserError> {
        let name = request.name.to_string();

        if is_reserved_name(&name) || self.user_repo.is_name_taken(&name).await? {
            return Ok(());
        }
        Err(UserError::NotFound(name))
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

    async fn list_organizations(
        &self,
        request: ListUserOrganizationsRequest,
    ) -> Result<Vec<OrganizationResponse>, UserError> {
        let user_name = request.user_name.to_string();
        let user = self
            .user_repo
            .get(&user_name)
            .await?
            .ok_or_else(|| UserError::NotFound(user_name))?;

        let orgs = self.org_repo.list_by_user_id(user.id).await?;
        Ok(orgs.into_iter().map(|o| o.into()).collect())
    }
}
