use async_trait::async_trait;

use crate::{
    client::{Git2Client, GitClient, ImageClient, ImageClientImpl, R2Client, R2ClientImpl},
    dto::{
        FollowUserRequest, GetCurrentUserRequest, GetCurrentUserResponse, GetUserRequest,
        HasUserRequest, ListUserCommitsRequest, ListUserContributedRepositoriesRequest,
        ListUserOrganizationsRequest, ListUserRepositoriesRequest, ListUserReviewsRequest,
        ListUserStarredRepositoriesRequest, MAX_PER_PAGE_LIMIT, Page, ReviewResponse,
        UnfollowUserRequest, UpdateCurrentUserImageRequest, UpdateCurrentUserRequest,
        UserCommitResponse, UserOrganizationResponse, UserRepositoryResponse, UserResponse,
    },
    error::{ConflictError, NotFoundError, OptionNotFoundExt, UserError},
    repository::{
        CommitRepository, OrganizationRepository, PgCommitRepository, PgOrganizationRepository,
        PgReviewRepository, PgUserRepository, ReviewRepository, UserRepository,
    },
    util::{
        auth::{is_offensive_name, is_reserved_name, validate_name},
        cursor,
    },
};

/// User accounts and their public profiles: the authenticated caller's own
/// profile (with emails and org memberships), name lookups and rename, avatar
/// uploads, and viewer-scoped listings of a user's repositories, stars,
/// contributions, organizations, reviews, and commits.
#[async_trait]
pub trait UserService: Send + Sync + 'static {
    /// Assembles the authenticated caller's full profile from `request.user_id`,
    /// bundling the user's own fields together with their email addresses and
    /// org memberships (capped at [`MAX_PER_PAGE_LIMIT`]).
    ///
    /// # Errors
    /// - [`UserError::NotFound`] when no user exists for `request.user_id`.
    async fn get_current_user(
        &self,
        request: GetCurrentUserRequest,
    ) -> Result<GetCurrentUserResponse, UserError>;

    /// Updates the authenticated caller's profile and returns the new state.
    ///
    /// When `request.name` is set and differs from the current name, the name is
    /// rejected if it is reserved or already taken, otherwise the user is
    /// renamed. The rename is applied to the on-disk bare-repo owner directory
    /// *before* the DB write so a filesystem failure aborts the rename; if the DB
    /// write then fails, the directory rename is reverted. A name equal to the
    /// current name is treated as a no-op rename.
    ///
    /// # Errors
    /// - [`UserError::Conflict`] when the new name is reserved or already taken.
    /// - [`UserError::NotFound`] when the user no longer exists.
    async fn update_current_user(
        &self,
        request: UpdateCurrentUserRequest,
    ) -> Result<UserResponse, UserError>;

    /// Sets the authenticated caller's avatar.
    ///
    /// Converts the uploaded bytes to WebP, stores them in object storage under
    /// `users/{user_id}.webp`, and bumps the user's `image_updated_at`.
    async fn update_current_user_image(
        &self,
        request: UpdateCurrentUserImageRequest,
    ) -> Result<(), UserError>;

    /// Reports whether `request.name` is unavailable as a new user name.
    ///
    /// Returns `Ok(())` when the name is reserved or already taken (i.e. cannot
    /// be claimed). Note this conflates "exists" with "reserved"; it does not
    /// confirm that a real user owns the name.
    ///
    /// # Errors
    /// - [`UserError::NotFound`] when the name is both free and not reserved.
    async fn has_user(&self, request: HasUserRequest) -> Result<(), UserError>;

    /// Returns the public profile for the user named `request.user_name`.
    ///
    /// # Errors
    /// - [`UserError::NotFound`] when no such user exists.
    async fn get_user(&self, request: GetUserRequest) -> Result<UserResponse, UserError>;

    async fn follow_user(&self, request: FollowUserRequest) -> Result<(), UserError>;

    async fn unfollow_user(&self, request: UnfollowUserRequest) -> Result<(), UserError>;

    /// Lists the repositories owned by `request.user_name`, paginated.
    ///
    /// Visibility is filtered in SQL by `request.viewer_id`. Each row carries the
    /// profile user's own commit count and last-commit time in that repo (NULL
    /// when they have no commits there).
    ///
    /// # Errors
    /// - [`UserError::NotFound`] when no such user exists.
    async fn list_repositories(
        &self,
        request: ListUserRepositoriesRequest,
    ) -> Result<Page<UserRepositoryResponse>, UserError>;

    /// Lists the repositories starred by `request.user_name`, paginated.
    ///
    /// Visibility is filtered in SQL by `request.viewer_id`. Commit stats are not
    /// surfaced for starred repos.
    ///
    /// # Errors
    /// - [`UserError::NotFound`] when no such user exists.
    async fn list_starred_repositories(
        &self,
        request: ListUserStarredRepositoriesRequest,
    ) -> Result<Page<UserRepositoryResponse>, UserError>;

    /// Lists repositories `request.user_name` has contributed to, paginated.
    ///
    /// Visibility is filtered in SQL by `request.viewer_id`, optionally bounded by
    /// `request.from`. Each row carries the user's commit count and last-commit
    /// time in that repo.
    ///
    /// # Errors
    /// - [`UserError::NotFound`] when no such user exists.
    async fn list_contributed_repositories(
        &self,
        request: ListUserContributedRepositoriesRequest,
    ) -> Result<Page<UserRepositoryResponse>, UserError>;

    /// Lists the organizations `request.user_name` is a member of, paginated.
    ///
    /// # Errors
    /// - [`UserError::NotFound`] when no such user exists.
    async fn list_organizations(
        &self,
        request: ListUserOrganizationsRequest,
    ) -> Result<Page<UserOrganizationResponse>, UserError>;

    /// Lists reviews authored by `request.user_name`, paginated.
    ///
    /// Results are scoped to what `request.viewer_id` may see and may be further
    /// filtered by status and by a specific `owner`/`repo`. Does not first verify
    /// the user exists.
    async fn list_reviews(
        &self,
        request: ListUserReviewsRequest,
    ) -> Result<Page<ReviewResponse>, UserError>;

    /// Lists commits authored by `request.user_name`, paginated.
    ///
    /// Visibility is decided in SQL: each row carries a `viewer_has_access`
    /// boolean mirroring repository access rules (public, viewer-owned, or
    /// org-owned with the viewer as a member). Commits the viewer cannot access
    /// are returned as redacted stubs rather than omitted. Bounded by the
    /// optional `request.from`/`request.to` range.
    ///
    /// # Errors
    /// - [`UserError::NotFound`] when no such user exists.
    async fn list_commits(
        &self,
        request: ListUserCommitsRequest,
    ) -> Result<Page<UserCommitResponse>, UserError>;
}

#[derive(Debug, Clone)]
pub struct UserServiceImpl<U, O, V, C, I, R2, G>
where
    U: UserRepository,
    O: OrganizationRepository,
    V: ReviewRepository,
    C: CommitRepository,
    I: ImageClient,
    R2: R2Client,
    G: GitClient,
{
    user_repo: U,
    org_repo: O,
    review_repo: V,
    commit_repo: C,
    image_client: I,
    r2_client: R2,
    git_client: G,
}

impl
    UserServiceImpl<
        PgUserRepository,
        PgOrganizationRepository,
        PgReviewRepository,
        PgCommitRepository,
        ImageClientImpl,
        R2ClientImpl,
        Git2Client,
    >
{
    pub fn new(
        user_repo: PgUserRepository,
        org_repo: PgOrganizationRepository,
        review_repo: PgReviewRepository,
        commit_repo: PgCommitRepository,
        image_client: ImageClientImpl,
        r2_client: R2ClientImpl,
        git_client: Git2Client,
    ) -> Self {
        Self {
            user_repo,
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
impl<U, O, V, C, I, R2, G> UserService for UserServiceImpl<U, O, V, C, I, R2, G>
where
    U: UserRepository,
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
            followers: user.followers,
            following: user.following,
            user_follow: user.user_follow,
        })
    }

    async fn update_current_user(
        &self,
        request: UpdateCurrentUserRequest,
    ) -> Result<UserResponse, UserError> {
        let rename: Option<(String, String)> = match request.name {
            Some(n) => {
                let new_name = n.to_string();
                validate_name("user name", &new_name)?;
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

                    // A concurrent rename can claim the handle between the
                    // is_name_taken check and this UPDATE; the unique index turns
                    // that race into a clean conflict instead of a 500.
                    if err.is_unique_violation() {
                        return Err(ConflictError::new(
                            "user name",
                            format!("{new_name} is already taken"),
                        )
                        .into());
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

        if is_reserved_name(&name)
            || is_offensive_name(&name)
            || self.user_repo.is_name_taken(&name).await?
        {
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

    async fn follow_user(&self, request: FollowUserRequest) -> Result<(), UserError> {
        let user_name = request.user_name.to_string();
        match self
            .user_repo
            .follow(&user_name, request.follower_id)
            .await?
        {
            Some(_) => Ok(()),
            None => {
                if self.user_repo.get(&user_name).await?.is_none() {
                    return Err(NotFoundError::new("user", &user_name).into());
                }
                Err(ConflictError::new("user follow", &user_name).into())
            }
        }
    }

    async fn unfollow_user(&self, request: UnfollowUserRequest) -> Result<(), UserError> {
        let user_name = request.user_name.to_string();
        if self
            .user_repo
            .unfollow(&user_name, request.follower_id)
            .await?
        {
            return Ok(());
        }
        if self.user_repo.get(&user_name).await?.is_none() {
            return Err(NotFoundError::new("user", &user_name).into());
        }
        Err(ConflictError::new("user follow", &user_name).into())
    }

    async fn list_repositories(
        &self,
        request: ListUserRepositoriesRequest,
    ) -> Result<Page<UserRepositoryResponse>, UserError> {
        let user_name = request.user_name.to_string();
        let user = self
            .user_repo
            .get(&user_name)
            .await?
            .or_not_found("user", &user_name)?;

        // visibility is filtered in SQL based on viewer_id; commit stats are
        // the profile user's own commits (NULL when they have none in a repo)
        let (rows, next_cursor) = self
            .user_repo
            .list_repositories(
                user.id,
                request.viewer_id,
                request.cursor,
                request.limit as i64,
            )
            .await?;

        Ok(Page {
            data: rows
                .into_iter()
                .map(|(repo, count, last)| {
                    UserRepositoryResponse::from_repository(repo, count, last)
                })
                .collect(),
            next_cursor: next_cursor.as_ref().map(cursor::encode),
        })
    }

    async fn list_starred_repositories(
        &self,
        request: ListUserStarredRepositoriesRequest,
    ) -> Result<Page<UserRepositoryResponse>, UserError> {
        let user_name = request.user_name.to_string();
        let user = self
            .user_repo
            .get(&user_name)
            .await?
            .or_not_found("user", &user_name)?;

        // visibility is filtered in SQL based on viewer_id
        let (repositories, next_cursor) = self
            .user_repo
            .list_starred_repositories(
                user.id,
                request.viewer_id,
                request.cursor,
                request.limit as i64,
            )
            .await?;

        // commit stats are not surfaced for starred repos
        Ok(Page {
            data: repositories
                .into_iter()
                .map(|r| UserRepositoryResponse::from_repository(r, None, None))
                .collect(),
            next_cursor: next_cursor.as_ref().map(cursor::encode),
        })
    }

    async fn list_contributed_repositories(
        &self,
        request: ListUserContributedRepositoriesRequest,
    ) -> Result<Page<UserRepositoryResponse>, UserError> {
        let user_name = request.user_name.to_string();
        let user = self
            .user_repo
            .get(&user_name)
            .await?
            .or_not_found("user", &user_name)?;

        // visibility is filtered in SQL based on viewer_id
        let (rows, next_cursor) = self
            .user_repo
            .list_contributed_repositories(
                user.id,
                request.viewer_id,
                request.from,
                request.cursor,
                request.limit as i64,
            )
            .await?;

        Ok(Page {
            data: rows
                .into_iter()
                .map(|(repo, count, last)| {
                    UserRepositoryResponse::from_repository(repo, Some(count), Some(last))
                })
                .collect(),
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

#[cfg(test)]
mod tests {
    use bytes::Bytes;
    use uuid::Uuid;

    use super::{UserService, UserServiceImpl};
    use crate::{
        dto::{
            GetCurrentUserRequest, GetUserRequest, HasUserRequest, ListUserCommitsRequest,
            UpdateCurrentUserImageRequest, UpdateCurrentUserRequest,
        },
        error::{DatabaseError, UserError},
        service::{
            test_client::{MockGitClient, MockImageClient, MockR2Client},
            test_common::{create_commit, create_user},
            test_repository::{
                MockCommitRepository, MockOrganizationRepository, MockReviewRepository,
                MockUserRepository,
            },
        },
    };

    type Service = UserServiceImpl<
        MockUserRepository,
        MockOrganizationRepository,
        MockReviewRepository,
        MockCommitRepository,
        MockImageClient,
        MockR2Client,
        MockGitClient,
    >;

    fn create_service() -> Service {
        UserServiceImpl {
            user_repo: MockUserRepository::new(),
            org_repo: MockOrganizationRepository::new(),
            review_repo: MockReviewRepository::new(),
            commit_repo: MockCommitRepository::new(),
            image_client: MockImageClient::new(),
            r2_client: MockR2Client::new(),
            git_client: MockGitClient::default(),
        }
    }

    #[tokio::test]
    async fn has_user_reserved_name_is_ok() {
        let service = create_service();
        service
            .has_user(HasUserRequest::new("admin").unwrap())
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn has_user_existing_name_is_ok() {
        let mut service = create_service();
        service
            .user_repo
            .expect_is_name_taken()
            .returning(|_| Ok(true));
        service
            .has_user(HasUserRequest::new("bob").unwrap())
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn has_user_unknown_name_is_not_found() {
        let mut service = create_service();
        service
            .user_repo
            .expect_is_name_taken()
            .returning(|_| Ok(false));
        let err = service
            .has_user(HasUserRequest::new("bob").unwrap())
            .await
            .unwrap_err();
        assert!(matches!(err, UserError::NotFound(_)));
    }

    #[tokio::test]
    async fn get_user_returns_response() {
        let mut service = create_service();
        service
            .user_repo
            .expect_get()
            .returning(|_| Ok(Some(create_user("alice"))));
        let resp = service
            .get_user(GetUserRequest::new("alice").unwrap())
            .await
            .unwrap();
        assert_eq!(resp.name, "alice");
    }

    #[tokio::test]
    async fn get_user_missing_is_not_found() {
        let mut service = create_service();
        service.user_repo.expect_get().returning(|_| Ok(None));
        let err = service
            .get_user(GetUserRequest::new("ghost").unwrap())
            .await
            .unwrap_err();
        assert!(matches!(err, UserError::NotFound(_)));
    }

    #[tokio::test]
    async fn get_current_user_assembles_profile() {
        let user = create_user("alice");
        let uid = user.id;

        let mut service = create_service();
        service
            .user_repo
            .expect_get_by_id()
            .returning(move |_| Ok(Some(user.clone())));
        service
            .user_repo
            .expect_list_emails()
            .returning(|_| Ok(vec![]));
        service
            .org_repo
            .expect_list_memberships_by_user_id()
            .returning(|_, _, _| Ok((vec![], None)));

        let resp = service
            .get_current_user(GetCurrentUserRequest::new(uid))
            .await
            .unwrap();
        assert_eq!(resp.id, uid);
        assert_eq!(resp.name, "alice");
        assert!(resp.emails.is_empty());
        assert!(resp.memberships.is_empty());
    }

    #[tokio::test]
    async fn update_current_user_rejects_reserved_name() {
        // No repo/git expectations: the reserved check returns before any call.
        let service = create_service();
        let req =
            UpdateCurrentUserRequest::new(Uuid::new_v4(), Some("admin"), None, None, None, None)
                .unwrap();
        let err = service.update_current_user(req).await.unwrap_err();
        assert!(matches!(err, UserError::Conflict(_)));
    }

    #[tokio::test]
    async fn update_current_user_rejects_taken_name() {
        let mut service = create_service();
        service
            .user_repo
            .expect_is_name_taken()
            .returning(|_| Ok(true));
        let req =
            UpdateCurrentUserRequest::new(Uuid::new_v4(), Some("bob"), None, None, None, None)
                .unwrap();
        let err = service.update_current_user(req).await.unwrap_err();
        assert!(matches!(err, UserError::Conflict(_)));
    }

    #[tokio::test]
    async fn update_current_user_renames_owner_dir_and_updates() {
        let mut service = create_service();
        service
            .user_repo
            .expect_is_name_taken()
            .returning(|_| Ok(false));
        service
            .user_repo
            .expect_get_by_id()
            .returning(|_| Ok(Some(create_user("alice"))));
        service
            .user_repo
            .expect_update()
            .returning(|_, name, _, _, _, _| Ok(create_user(name.as_deref().unwrap_or("alice"))));

        let req =
            UpdateCurrentUserRequest::new(Uuid::new_v4(), Some("bob"), None, None, None, None)
                .unwrap();
        let resp = service.update_current_user(req).await.unwrap();
        assert_eq!(resp.name, "bob");
        // The on-disk owner directory is renamed alice -> bob before the DB write.
        assert_eq!(
            service.git_client.renames(),
            vec![("alice".into(), "bob".into())]
        );
    }

    #[tokio::test]
    async fn update_current_user_skips_rename_when_name_unchanged() {
        let mut service = create_service();
        service
            .user_repo
            .expect_is_name_taken()
            .returning(|_| Ok(false));
        service
            .user_repo
            .expect_get_by_id()
            .returning(|_| Ok(Some(create_user("alice"))));
        service
            .user_repo
            .expect_update()
            .returning(|_, _, _, _, _, _| Ok(create_user("alice")));

        let req =
            UpdateCurrentUserRequest::new(Uuid::new_v4(), Some("alice"), None, None, None, None)
                .unwrap();
        service.update_current_user(req).await.unwrap();
        // Same name => no directory rename.
        assert!(service.git_client.renames().is_empty());
    }

    #[tokio::test]
    async fn update_current_user_reverts_rename_on_update_failure() {
        let mut service = create_service();
        service
            .user_repo
            .expect_is_name_taken()
            .returning(|_| Ok(false));
        service
            .user_repo
            .expect_get_by_id()
            .returning(|_| Ok(Some(create_user("alice"))));
        service
            .user_repo
            .expect_update()
            .returning(|_, _, _, _, _, _| Err(DatabaseError::RowNotFound));

        let req =
            UpdateCurrentUserRequest::new(Uuid::new_v4(), Some("bob"), None, None, None, None)
                .unwrap();
        let err = service.update_current_user(req).await.unwrap_err();
        assert!(matches!(err, UserError::DatabaseError(_)));
        // Forward rename then a compensating revert once the DB write failed.
        assert_eq!(
            service.git_client.renames(),
            vec![
                ("alice".into(), "bob".into()),
                ("bob".into(), "alice".into())
            ]
        );
    }

    #[tokio::test]
    async fn update_current_user_image_uploads_and_touches() {
        let mut service = create_service();
        service
            .image_client
            .expect_convert_to_webp()
            .returning(|_| Ok(Bytes::from_static(b"webp")));
        service
            .r2_client
            .expect_upload_object()
            .returning(|_, _| Ok(()));
        service.user_repo.expect_touch_image().returning(|_| Ok(()));

        let req = UpdateCurrentUserImageRequest::new(Uuid::new_v4(), Bytes::from_static(b"png"));
        service.update_current_user_image(req).await.unwrap();
    }

    #[tokio::test]
    async fn list_commits_maps_visible_and_redacted() {
        let mut service = create_service();
        service
            .user_repo
            .expect_get()
            .returning(|_| Ok(Some(create_user("alice"))));
        service
            .commit_repo
            .expect_list_by_user()
            .returning(|_, _, _, _, _, _| {
                Ok((
                    vec![(create_commit("a"), true), (create_commit("b"), false)],
                    None,
                ))
            });

        let req =
            ListUserCommitsRequest::new("alice", Some(Uuid::new_v4()), None, None, None, None)
                .unwrap();
        let page = service.list_commits(req).await.unwrap();
        assert_eq!(page.data.len(), 2);
        // Accessible commit is fully populated...
        assert!(!page.data[0].redacted);
        assert_eq!(page.data[0].owner_name.as_deref(), Some("alice"));
        // ...inaccessible commit is redacted to a stub.
        assert!(page.data[1].redacted);
        assert!(page.data[1].owner_name.is_none());
    }
}
