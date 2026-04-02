use async_trait::async_trait;

use crate::{
    dto::{
        GetCurrentUserRequest, GetCurrentUserSettingsRequest, GetUserRequest, HasUserRequest,
        ListUserOrganizationsRequest, ListUserRepositoriesRequest, ListUserReviewsRequest,
        OrganizationResponse, RepositoryResponse, ReviewResponse, UpdateCurrentUserRequest,
        UpdateCurrentUserSettingsRequest, UserResponse, UserSettingsResponse,
    },
    error::{ConflictError, NotFoundError, OptionNotFoundExt, UserError},
    model::UserSettings,
    repository::{
        OrganizationRepository, OrganizationRepositoryImpl, RepositoryRepository,
        RepositoryRepositoryImpl, ReviewRepository, ReviewRepositoryImpl, UserRepository,
        UserRepositoryImpl,
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

    async fn list_reviews(
        &self,
        request: ListUserReviewsRequest,
    ) -> Result<Vec<ReviewResponse>, UserError>;

    async fn get_current_user_settings(
        &self,
        request: GetCurrentUserSettingsRequest,
    ) -> Result<UserSettingsResponse, UserError>;

    async fn update_current_user_settings(
        &self,
        request: UpdateCurrentUserSettingsRequest,
    ) -> Result<UserSettingsResponse, UserError>;
}

#[derive(Debug, Clone)]
pub struct UserServiceImpl<U, R, O, V>
where
    U: UserRepository,
    R: RepositoryRepository,
    O: OrganizationRepository,
    V: ReviewRepository,
{
    user_repo: U,
    repo_repo: R,
    org_repo: O,
    review_repo: V,
}

impl
    UserServiceImpl<
        UserRepositoryImpl,
        RepositoryRepositoryImpl,
        OrganizationRepositoryImpl,
        ReviewRepositoryImpl,
    >
{
    pub fn new(
        user_repo: UserRepositoryImpl,
        repo_repo: RepositoryRepositoryImpl,
        org_repo: OrganizationRepositoryImpl,
        review_repo: ReviewRepositoryImpl,
    ) -> Self {
        Self {
            user_repo,
            repo_repo,
            org_repo,
            review_repo,
        }
    }
}

#[crate::instrument_all]
#[async_trait]
impl<U, R, O, V> UserService for UserServiceImpl<U, R, O, V>
where
    U: UserRepository,
    R: RepositoryRepository,
    O: OrganizationRepository,
    V: ReviewRepository,
{
    async fn get_current_user(
        &self,
        request: GetCurrentUserRequest,
    ) -> Result<UserResponse, UserError> {
        let user = self
            .user_repo
            .get_by_id(request.user_id)
            .await?
            .or_not_found("user", request.user_id)?;
        Ok(user.into())
    }

    async fn update_current_user(
        &self,
        request: UpdateCurrentUserRequest,
    ) -> Result<UserResponse, UserError> {
        let name = request.name.to_string();

        if is_reserved_name(&name) {
            return Err(ConflictError::new("user name", format!("{name} is reserved")).into());
        }

        if self.user_repo.is_name_taken(&name).await? {
            return Err(ConflictError::new("user name", format!("{name} is already taken")).into());
        }

        let user = self.user_repo.update(request.user_id, &name).await?;
        Ok(user.into())
    }

    async fn has_user(&self, request: HasUserRequest) -> Result<(), UserError> {
        let name = request.name.to_string();

        if is_reserved_name(&name) || self.user_repo.is_name_taken(&name).await? {
            return Ok(());
        }
        Err(NotFoundError::new("user", name).into())
    }

    async fn get_user(&self, request: GetUserRequest) -> Result<UserResponse, UserError> {
        let user_name = request.user_name.to_string();
        let user = self
            .user_repo
            .get(&user_name)
            .await?
            .or_not_found("user", &user_name)?;
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
            .or_not_found("user", &user_name)?;

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
            .or_not_found("user", &user_name)?;

        let orgs = self.org_repo.list_by_user_id(user.id).await?;
        Ok(orgs.into_iter().map(|o| o.into()).collect())
    }

    async fn list_reviews(
        &self,
        request: ListUserReviewsRequest,
    ) -> Result<Vec<ReviewResponse>, UserError> {
        let reviews = self
            .review_repo
            .get_reviews_by_user(
                request.user_name.as_ref(),
                request.viewer_id,
                request.status,
                request.owner,
                request.repo,
            )
            .await?;

        Ok(reviews.into_iter().map(ReviewResponse::from).collect())
    }

    async fn get_current_user_settings(
        &self,
        request: GetCurrentUserSettingsRequest,
    ) -> Result<UserSettingsResponse, UserError> {
        let settings = self.user_repo.get_settings(request.user_id).await?;
        Ok(settings.unwrap_or_default().into())
    }

    async fn update_current_user_settings(
        &self,
        request: UpdateCurrentUserSettingsRequest,
    ) -> Result<UserSettingsResponse, UserError> {
        let patch = UserSettings {
            repos: request.repos.unwrap_or_default(),
        };
        let settings = self
            .user_repo
            .update_settings(request.user_id, patch)
            .await?
            .or_not_found("user", request.user_id)?;
        Ok(settings.into())
    }
}
