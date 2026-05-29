use async_trait::async_trait;

use crate::{
    client::{Git2Client, GitClient, ImageClient, ImageClientImpl, R2Client, R2ClientImpl},
    dto::{
        GetCurrentUserRequest, GetCurrentUserResponse, GetUserRequest, HasUserRequest,
        ListUserCommitsRequest, ListUserOrganizationsRequest, ListUserRepositoriesRequest,
        ListUserReviewsRequest, ListUserStarredRepositoriesRequest, MAX_PER_PAGE_LIMIT, Page,
        RepositoryResponse, ReviewResponse, UpdateCurrentUserImageRequest,
        UpdateCurrentUserRequest, UserCommitResponse, UserOrganizationResponse, UserResponse,
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

    async fn list_starred_repositories(
        &self,
        request: ListUserStarredRepositoriesRequest,
    ) -> Result<Page<RepositoryResponse>, UserError>;

    async fn list_organizations(
        &self,
        request: ListUserOrganizationsRequest,
    ) -> Result<Page<UserOrganizationResponse>, UserError>;

    async fn list_reviews(
        &self,
        request: ListUserReviewsRequest,
    ) -> Result<Page<ReviewResponse>, UserError>;

    async fn list_commits(
        &self,
        request: ListUserCommitsRequest,
    ) -> Result<Page<UserCommitResponse>, UserError>;
}

#[derive(Debug, Clone)]
pub struct UserServiceImpl<U, R, O, V, C, I, R2, G>
where
    U: UserRepository,
    R: RepositoryRepository,
    O: OrganizationRepository,
    V: ReviewRepository,
    C: CommitRepository,
    I: ImageClient,
    R2: R2Client,
    G: GitClient,
{
    user_repo: U,
    repo_repo: R,
    org_repo: O,
    review_repo: V,
    commit_repo: C,
    image_client: I,
    r2_client: R2,
    git_client: G,
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
        Git2Client,
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
        git_client: Git2Client,
    ) -> Self {
        Self {
            user_repo,
            repo_repo,
            org_repo,
            review_repo,
            commit_repo,
            image_client,
            r2_client,
            git_client,
        }
    }
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl<U, R, O, V, C, I, R2, G> UserService for UserServiceImpl<U, R, O, V, C, I, R2, G>
where
    U: UserRepository,
    R: RepositoryRepository,
    O: OrganizationRepository,
    V: ReviewRepository,
    C: CommitRepository,
    I: ImageClient,
    R2: R2Client,
    G: GitClient,
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
        let emails = self
            .user_repo
            .list_emails(user.id)
            .await?
            .into_iter()
            .map(Into::into)
            .collect();
        Ok(GetCurrentUserResponse {
            id: user.id,
            name: user.name,
            emails: emails,
            memberships: memberships,
            location: user.location,
            readme: user.readme,
            links: user.links,
            display_name: user.display_name,
            created_at: user.created_at,
            image_updated_at: user.image_updated_at,
        })
    }

    async fn update_current_user(
        &self,
        request: UpdateCurrentUserRequest,
    ) -> Result<UserResponse, UserError> {
        let rename: Option<(String, String)> = match request.name {
            Some(n) => {
                let new_name = n.to_string();
                if is_reserved_name(&new_name) {
                    return Err(
                        ConflictError::new("user name", format!("{new_name} is reserved")).into(),
                    );
                }
                if self.user_repo.is_name_taken(&new_name).await? {
                    return Err(ConflictError::new(
                        "user name",
                        format!("{new_name} is already taken"),
                    )
                    .into());
                }
                let old_name = self
                    .user_repo
                    .get_by_id(request.user_id)
                    .await?
                    .or_not_found("user", request.user_id)?
                    .name;
                (old_name != new_name).then_some((old_name, new_name))
            }
            None => None,
        };

        // Move the on-disk bare-repo directory before the DB write so a
        // filesystem failure aborts the rename without leaving the two out of
        // sync.
        if let Some((old_name, new_name)) = &rename {
            self.git_client.rename_owner(old_name, new_name).await?;
        }

        let update_result = self
            .user_repo
            .update(
                request.user_id,
                rename.as_ref().map(|(_, new_name)| new_name.clone()),
                request.location,
                request.readme,
                request.links,
                request.display_name,
            )
            .await;

        let user = match update_result {
            Ok(user) => user,
            Err(err) => {
                if let Some((old_name, new_name)) = &rename {
                    if let Err(revert_err) = self.git_client.rename_owner(new_name, old_name).await
                    {
                        tracing::error!(
                            %revert_err,
                            old_name,
                            new_name,
                            "failed to revert owner directory rename after DB update failed"
                        );
                    }
                }
                return Err(err.into());
            }
        };
        Ok(user.into())
    }

    async fn update_current_user_image(
        &self,
        request: UpdateCurrentUserImageRequest,
    ) -> Result<(), UserError> {
        let webp_bytes = self.image_client.convert_to_webp(request.bytes).await?;
        let key = format!("users/{}.webp", request.user_id);
        self.r2_client.upload_object(&key, webp_bytes).await?;
        self.user_repo.touch_image(request.user_id).await?;
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
            .list_by_owner(
                &user_name,
                request.viewer_id,
                request.cursor,
                request.limit as i64,
            )
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

    async fn list_starred_repositories(
        &self,
        request: ListUserStarredRepositoriesRequest,
    ) -> Result<Page<RepositoryResponse>, UserError> {
        let user_name = request.user_name.to_string();
        let user = self
            .user_repo
            .get(&user_name)
            .await?
            .or_not_found("user", &user_name)?;

        let (repositories, next_cursor) = self
            .user_repo
            .list_starred_repositories(
                user.id,
                request.viewer_id,
                request.cursor,
                request.limit as i64,
            )
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
    ) -> Result<Page<UserOrganizationResponse>, UserError> {
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
    ) -> Result<Page<UserCommitResponse>, UserError> {
        let user_name = request.user_name.to_string();
        let user = self
            .user_repo
            .get(&user_name)
            .await?
            .or_not_found("user", &user_name)?;

        // Visibility is decided in SQL: each row carries a `viewer_has_access`
        // boolean reflecting the same rules as `verify_authorized_for_repository`
        // (public, user-owned by viewer, or org-owned with viewer as member).
        // Rows the viewer cannot access are returned as redacted stubs.
        let (rows, next_cursor) = self
            .commit_repo
            .list_by_user(
                user.id,
                request.viewer_id,
                request.from,
                request.to,
                request.cursor,
                request.limit as i64,
            )
            .await?;

        let data = rows
            .into_iter()
            .map(|(commit, has_access)| {
                if has_access {
                    UserCommitResponse::visible(commit)
                } else {
                    UserCommitResponse::redacted(&commit)
                }
            })
            .collect();

        Ok(Page {
            data,
            next_cursor: next_cursor.as_ref().map(cursor::encode),
        })
    }
}
