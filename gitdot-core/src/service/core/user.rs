use async_trait::async_trait;
use chrono::{Duration, Utc};

use crate::{
    client::{ImageClient, ImageClientImpl, R2Client, R2ClientImpl},
    dto::{
        CommitResponse, GetCurrentUserRequest, GetCurrentUserResponse, GetUserRequest,
        HasUserRequest, ListUserCommitsRequest, ListUserOrganizationsRequest,
        ListUserRepositoriesRequest, ListUserReviewsRequest, ListUserStarsRequest,
        MAX_PER_PAGE_LIMIT, OrganizationMemberResponse, Page, RepositoryResponse, ReviewResponse,
        UpdateCurrentUserImageRequest, UpdateCurrentUserRequest, UserResponse,
    },
    error::{ConflictError, NotFoundError, OptionNotFoundExt, UserError},
    repository::{
        CommitRepository, CommitRepositoryImpl, OrganizationRepository, OrganizationRepositoryImpl,
        RepositoryRepository, RepositoryRepositoryImpl, ReviewRepository, ReviewRepositoryImpl,
        UserRepository, UserRepositoryImpl,
    },
    util::{auth::is_reserved_name, cursor},
};

#[async_trait]
pub trait UserService: Send + Sync + 'static {
    async fn get_current_user(
        &self,
        request: GetCurrentUserRequest,
    ) -> Result<GetCurrentUserResponse, UserError>;

    async fn update_current_user(
        &self,
        request: UpdateCurrentUserRequest,
    ) -> Result<UserResponse, UserError>;

    async fn update_current_user_image(
        &self,
        request: UpdateCurrentUserImageRequest,
    ) -> Result<(), UserError>;

    async fn has_user(&self, request: HasUserRequest) -> Result<(), UserError>;

    async fn get_user(&self, request: GetUserRequest) -> Result<UserResponse, UserError>;

    async fn list_repositories(
        &self,
        request: ListUserRepositoriesRequest,
    ) -> Result<Page<RepositoryResponse>, UserError>;

    async fn list_stars(
        &self,
        request: ListUserStarsRequest,
    ) -> Result<Page<RepositoryResponse>, UserError>;

    async fn list_organizations(
        &self,
        request: ListUserOrganizationsRequest,
    ) -> Result<Page<OrganizationMemberResponse>, UserError>;

    async fn list_reviews(
        &self,
        request: ListUserReviewsRequest,
    ) -> Result<Page<ReviewResponse>, UserError>;

    async fn list_commits(
        &self,
        request: ListUserCommitsRequest,
    ) -> Result<Vec<CommitResponse>, UserError>;
}

#[derive(Debug, Clone)]
pub struct UserServiceImpl<U, R, O, V, C, I, R2>
where
    U: UserRepository,
    R: RepositoryRepository,
    O: OrganizationRepository,
    V: ReviewRepository,
    C: CommitRepository,
    I: ImageClient,
    R2: R2Client,
{
    user_repo: U,
    repo_repo: R,
    org_repo: O,
    review_repo: V,
    commit_repo: C,
    image_client: I,
    r2_client: R2,
}

impl
    UserServiceImpl<
        UserRepositoryImpl,
        RepositoryRepositoryImpl,
        OrganizationRepositoryImpl,
        ReviewRepositoryImpl,
        CommitRepositoryImpl,
        ImageClientImpl,
        R2ClientImpl,
    >
{
    pub fn new(
        user_repo: UserRepositoryImpl,
        repo_repo: RepositoryRepositoryImpl,
        org_repo: OrganizationRepositoryImpl,
        review_repo: ReviewRepositoryImpl,
        commit_repo: CommitRepositoryImpl,
        image_client: ImageClientImpl,
        r2_client: R2ClientImpl,
    ) -> Self {
        Self {
            user_repo,
            repo_repo,
            org_repo,
            review_repo,
            commit_repo,
            image_client,
            r2_client,
        }
    }
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl<U, R, O, V, C, I, R2> UserService for UserServiceImpl<U, R, O, V, C, I, R2>
where
    U: UserRepository,
    R: RepositoryRepository,
    O: OrganizationRepository,
    V: ReviewRepository,
    C: CommitRepository,
    I: ImageClient,
    R2: R2Client,
{
    async fn get_current_user(
        &self,
        request: GetCurrentUserRequest,
    ) -> Result<GetCurrentUserResponse, UserError> {
        let user = self
            .user_repo
            .get_by_id(request.user_id)
            .await?
            .or_not_found("user", request.user_id)?;
        let (memberships, _) = self
            .org_repo
            .list_memberships_by_user_id(user.id, None, MAX_PER_PAGE_LIMIT as i64)
            .await?;
        let memberships = memberships.into_iter().map(Into::into).collect();
        Ok(GetCurrentUserResponse {
            user: user.into(),
            memberships,
        })
    }

    async fn update_current_user(
        &self,
        request: UpdateCurrentUserRequest,
    ) -> Result<UserResponse, UserError> {
        let name: Option<String> = match request.name {
            Some(n) => {
                let name = n.to_string();
                if is_reserved_name(&name) {
                    return Err(
                        ConflictError::new("user name", format!("{name} is reserved")).into(),
                    );
                }
                if self.user_repo.is_name_taken(&name).await? {
                    return Err(ConflictError::new(
                        "user name",
                        format!("{name} is already taken"),
                    )
                    .into());
                }
                Some(name)
            }
            None => None,
        };

        let user = self
            .user_repo
            .update(
                request.user_id,
                name,
                request.location,
                request.readme,
                request.links,
                request.display_name,
            )
            .await?;
        Ok(user.into())
    }

    async fn update_current_user_image(
        &self,
        request: UpdateCurrentUserImageRequest,
    ) -> Result<(), UserError> {
        let webp_bytes = self.image_client.convert_to_webp(request.bytes).await?;
        let key = format!("users/{}.webp", request.user_id);
        self.r2_client.upload_object(&key, webp_bytes).await?;
        Ok(())
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
    ) -> Result<Page<RepositoryResponse>, UserError> {
        let user_name = request.user_name.to_string();
        let user = self
            .user_repo
            .get(&user_name)
            .await?
            .or_not_found("user", &user_name)?;

        let (repositories, next_cursor) = self
            .repo_repo
            .list_by_owner(&user_name, request.cursor, request.limit as i64)
            .await?;

        let is_owner = request.viewer_id.map(|id| id == user.id).unwrap_or(false);
        let repositories = if is_owner {
            repositories
        } else {
            repositories.into_iter().filter(|r| r.is_public()).collect()
        };

        Ok(Page {
            data: repositories.into_iter().map(|r| r.into()).collect(),
            next_cursor: next_cursor.as_ref().map(cursor::encode),
        })
    }

    async fn list_stars(
        &self,
        request: ListUserStarsRequest,
    ) -> Result<Page<RepositoryResponse>, UserError> {
        let user_name = request.user_name.to_string();
        let user = self
            .user_repo
            .get(&user_name)
            .await?
            .or_not_found("user", &user_name)?;

        let (repositories, next_cursor) = self
            .user_repo
            .list_starred_repositories(user.id, request.cursor, request.limit as i64)
            .await?;

        let is_owner = request.viewer_id.map(|id| id == user.id).unwrap_or(false);
        let repositories = if is_owner {
            repositories
        } else {
            repositories.into_iter().filter(|r| r.is_public()).collect()
        };

        Ok(Page {
            data: repositories.into_iter().map(|r| r.into()).collect(),
            next_cursor: next_cursor.as_ref().map(cursor::encode),
        })
    }

    async fn list_organizations(
        &self,
        request: ListUserOrganizationsRequest,
    ) -> Result<Page<OrganizationMemberResponse>, UserError> {
        let user_name = request.user_name.to_string();
        let user = self
            .user_repo
            .get(&user_name)
            .await?
            .or_not_found("user", &user_name)?;

        let (memberships, next_cursor) = self
            .org_repo
            .list_memberships_by_user_id(user.id, request.cursor, request.limit as i64)
            .await?;
        Ok(Page {
            data: memberships.into_iter().map(|m| m.into()).collect(),
            next_cursor: next_cursor.as_ref().map(cursor::encode),
        })
    }

    async fn list_reviews(
        &self,
        request: ListUserReviewsRequest,
    ) -> Result<Page<ReviewResponse>, UserError> {
        let (reviews, next_cursor) = self
            .review_repo
            .list_reviews_by_user(
                request.user_name.as_ref(),
                request.viewer_id,
                request.status,
                request.owner,
                request.repo,
                request.cursor,
                request.limit as i64,
            )
            .await?;

        Ok(Page {
            data: reviews.into_iter().map(ReviewResponse::from).collect(),
            next_cursor: next_cursor.as_ref().map(cursor::encode),
        })
    }

    async fn list_commits(
        &self,
        request: ListUserCommitsRequest,
    ) -> Result<Vec<CommitResponse>, UserError> {
        let user_name = request.user_name.to_string();
        let user = self
            .user_repo
            .get(&user_name)
            .await?
            .or_not_found("user", &user_name)?;

        let now = Utc::now();
        let from = now - Duration::days(365);
        let commits = self.commit_repo.list_by_user(user.id, from, now).await?;
        Ok(commits.into_iter().map(CommitResponse::from).collect())
    }
}
