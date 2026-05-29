use async_trait::async_trait;

use crate::{
    dto::{
        AnswerAuthorizationRequest, CommentAuthorizationRequest, MigrationAuthorizationRequest,
        OrganizationAuthorizationRequest, OrganizationMemberAuthorizationRequest,
        QuestionAuthorizationRequest, RepositoryAuthorizationRequest,
        RepositoryCreationAuthorizationRequest, RepositoryPermission, ReviewAuthorizationRequest,
        ReviewCommentAuthorizationRequest, ReviewingAuthorizationRequest,
    },
    error::{AuthorizationError, OptionNotFoundExt},
    model::{OrganizationRole, RepositoryOwnerType},
    repository::{
        OrganizationRepository, OrganizationRepositoryImpl, QuestionRepository,
        QuestionRepositoryImpl, RepositoryRepository, RepositoryRepositoryImpl, ReviewRepository,
        ReviewRepositoryImpl, UserRepository, UserRepositoryImpl,
    },
};

#[async_trait]
pub trait AuthorizationService: Send + Sync + 'static {
    async fn verify_authorized_for_repository_creation(
        &self,
        request: RepositoryCreationAuthorizationRequest,
    ) -> Result<(), AuthorizationError>;

    async fn verify_authorized_for_repository(
        &self,
        request: RepositoryAuthorizationRequest,
    ) -> Result<(), AuthorizationError>;

    async fn verify_authorized_for_organization(
        &self,
        request: OrganizationAuthorizationRequest,
    ) -> Result<(), AuthorizationError>;

    async fn verify_authorized_for_organization_member(
        &self,
        request: OrganizationMemberAuthorizationRequest,
    ) -> Result<(), AuthorizationError>;

    async fn verify_authorized_for_question(
        &self,
        request: QuestionAuthorizationRequest,
    ) -> Result<(), AuthorizationError>;

    async fn verify_authorized_for_answer(
        &self,
        request: AnswerAuthorizationRequest,
    ) -> Result<(), AuthorizationError>;

    async fn verify_authorized_for_comment(
        &self,
        request: CommentAuthorizationRequest,
    ) -> Result<(), AuthorizationError>;

    async fn verify_authorized_for_review(
        &self,
        request: ReviewAuthorizationRequest,
    ) -> Result<(), AuthorizationError>;

    async fn verify_authorized_for_review_comment(
        &self,
        request: ReviewCommentAuthorizationRequest,
    ) -> Result<(), AuthorizationError>;

    async fn verify_authorized_for_reviewing(
        &self,
        request: ReviewingAuthorizationRequest,
    ) -> Result<(), AuthorizationError>;

    async fn verify_authorized_for_migration(
        &self,
        request: MigrationAuthorizationRequest,
    ) -> Result<(), AuthorizationError>;
}

#[derive(Debug, Clone)]
pub struct AuthorizationServiceImpl<O, R, Q, U, V>
where
    O: OrganizationRepository,
    R: RepositoryRepository,
    Q: QuestionRepository,
    U: UserRepository,
    V: ReviewRepository,
{
    org_repo: O,
    repo_repo: R,
    question_repo: Q,
    user_repo: U,
    review_repo: V,
}

impl
    AuthorizationServiceImpl<
        OrganizationRepositoryImpl,
        RepositoryRepositoryImpl,
        QuestionRepositoryImpl,
        UserRepositoryImpl,
        ReviewRepositoryImpl,
    >
{
    pub fn new(
        org_repo: OrganizationRepositoryImpl,
        repo_repo: RepositoryRepositoryImpl,
        question_repo: QuestionRepositoryImpl,
        user_repo: UserRepositoryImpl,
        review_repo: ReviewRepositoryImpl,
    ) -> Self {
        Self {
            org_repo,
            repo_repo,
            question_repo,
            user_repo,
            review_repo,
        }
    }
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl<O, R, Q, U, V> AuthorizationService for AuthorizationServiceImpl<O, R, Q, U, V>
where
    O: OrganizationRepository,
    R: RepositoryRepository,
    Q: QuestionRepository,
    U: UserRepository,
    V: ReviewRepository,
{
    async fn verify_authorized_for_repository_creation(
        &self,
        request: RepositoryCreationAuthorizationRequest,
    ) -> Result<(), AuthorizationError> {
        match request.owner_type {
            RepositoryOwnerType::User => {
                let user = self
                    .user_repo
                    .get_by_id(request.user_id)
                    .await?
                    .ok_or(AuthorizationError::Unauthorized)?;

                if user.name.to_lowercase() != request.owner.as_ref().to_lowercase() {
                    return Err(AuthorizationError::Unauthorized);
                }
            }
            RepositoryOwnerType::Organization => {
                let org = self
                    .org_repo
                    .get(request.owner.as_ref())
                    .await?
                    .ok_or(AuthorizationError::Unauthorized)?;

                let is_member = self.org_repo.is_member(org.id, request.user_id).await?;
                if !is_member {
                    return Err(AuthorizationError::Unauthorized);
                }
            }
        }
        Ok(())
    }

    async fn verify_authorized_for_repository(
        &self,
        request: RepositoryAuthorizationRequest,
    ) -> Result<(), AuthorizationError> {
        let repository = self
            .repo_repo
            .get(request.owner.as_ref(), request.repo.as_ref(), None)
            .await?
            .or_not_found(
                "repository",
                format!("{}/{}", request.owner.as_ref(), request.repo.as_ref()),
            )?;

        if repository.readonly && request.permission == RepositoryPermission::Write {
            return Err(AuthorizationError::ReadonlyRepository);
        }

        if request.permission == RepositoryPermission::Read && repository.is_public() {
            return Ok(());
        }

        let user_id = request.user_id.ok_or(AuthorizationError::Unauthorized)?;
        if repository.is_owned_by_user() {
            if repository.owner_id != user_id {
                return Err(AuthorizationError::Unauthorized);
            }
        } else {
            let authorized = if request.permission == RepositoryPermission::Admin {
                matches!(
                    self.org_repo
                        .get_member_role(request.owner.as_ref(), user_id)
                        .await?,
                    Some(OrganizationRole::Admin)
                )
            } else {
                self.org_repo
                    .is_member(repository.owner_id, user_id)
                    .await?
            };
            if !authorized {
                return Err(AuthorizationError::Unauthorized);
            }
        }

        Ok(())
    }

    async fn verify_authorized_for_organization(
        &self,
        request: OrganizationAuthorizationRequest,
    ) -> Result<(), AuthorizationError> {
        let role = self
            .org_repo
            .get_member_role(request.org_name.as_ref(), request.user_id)
            .await?;

        match role {
            Some(OrganizationRole::Admin) => Ok(()),
            _ => Err(AuthorizationError::Unauthorized),
        }
    }

    async fn verify_authorized_for_organization_member(
        &self,
        request: OrganizationMemberAuthorizationRequest,
    ) -> Result<(), AuthorizationError> {
        let member = self
            .org_repo
            .get_member(request.org_name.as_ref(), request.member_id)
            .await?
            .ok_or(AuthorizationError::Unauthorized)?;

        if member.user_id == request.auth_user_id {
            return Ok(());
        }

        let role = self
            .org_repo
            .get_member_role(request.org_name.as_ref(), request.auth_user_id)
            .await?;

        match role {
            Some(OrganizationRole::Admin) => Ok(()),
            _ => Err(AuthorizationError::Unauthorized),
        }
    }

    async fn verify_authorized_for_question(
        &self,
        request: QuestionAuthorizationRequest,
    ) -> Result<(), AuthorizationError> {
        let author_id = self
            .question_repo
            .get_question_author_id(
                request.owner.as_ref(),
                request.repo.as_ref(),
                request.number,
            )
            .await?
            .ok_or(AuthorizationError::Unauthorized)?;

        if author_id != request.user_id {
            return Err(AuthorizationError::Unauthorized);
        }

        Ok(())
    }

    async fn verify_authorized_for_answer(
        &self,
        request: AnswerAuthorizationRequest,
    ) -> Result<(), AuthorizationError> {
        let author_id = self
            .question_repo
            .get_answer_author_id(request.answer_id)
            .await?
            .ok_or(AuthorizationError::Unauthorized)?;

        if author_id != request.user_id {
            return Err(AuthorizationError::Unauthorized);
        }

        Ok(())
    }

    async fn verify_authorized_for_comment(
        &self,
        request: CommentAuthorizationRequest,
    ) -> Result<(), AuthorizationError> {
        let author_id = self
            .question_repo
            .get_comment_author_id(request.comment_id)
            .await?
            .ok_or(AuthorizationError::Unauthorized)?;

        if author_id != request.user_id {
            return Err(AuthorizationError::Unauthorized);
        }

        Ok(())
    }

    async fn verify_authorized_for_review(
        &self,
        request: ReviewAuthorizationRequest,
    ) -> Result<(), AuthorizationError> {
        let review = self
            .review_repo
            .get_review_by_number(
                request.owner.as_ref(),
                request.repo.as_ref(),
                request.number,
            )
            .await?
            .ok_or(AuthorizationError::Unauthorized)?;

        if review.author_id != request.user_id {
            return Err(AuthorizationError::Unauthorized);
        }

        Ok(())
    }

    async fn verify_authorized_for_review_comment(
        &self,
        request: ReviewCommentAuthorizationRequest,
    ) -> Result<(), AuthorizationError> {
        let comment = self
            .review_repo
            .get_comment(request.comment_id)
            .await?
            .ok_or(AuthorizationError::Unauthorized)?;

        if comment.author_id != request.user_id {
            return Err(AuthorizationError::Unauthorized);
        }

        Ok(())
    }

    async fn verify_authorized_for_reviewing(
        &self,
        request: ReviewingAuthorizationRequest,
    ) -> Result<(), AuthorizationError> {
        let review = self
            .review_repo
            .get_review_by_number(
                request.owner.as_ref(),
                request.repo.as_ref(),
                request.number,
            )
            .await?
            .ok_or(AuthorizationError::Unauthorized)?;

        let is_author = review.author_id == request.user_id;
        let reviewers = review.reviewers.unwrap_or_default();
        if !is_author && !reviewers.iter().any(|r| r.reviewer_id == request.user_id) {
            return Err(AuthorizationError::Unauthorized);
        }

        Ok(())
    }

    async fn verify_authorized_for_migration(
        &self,
        request: MigrationAuthorizationRequest,
    ) -> Result<(), AuthorizationError> {
        match request.owner_type {
            RepositoryOwnerType::User => {
                let user = self
                    .user_repo
                    .get_by_id(request.user_id)
                    .await?
                    .ok_or(AuthorizationError::Unauthorized)?;

                if user.name.to_lowercase() != request.owner_name.as_ref().to_lowercase() {
                    return Err(AuthorizationError::Unauthorized);
                }
            }
            RepositoryOwnerType::Organization => {
                let org = self
                    .org_repo
                    .get(request.owner_name.as_ref())
                    .await?
                    .ok_or(AuthorizationError::Unauthorized)?;

                let is_member = self.org_repo.is_member(org.id, request.user_id).await?;
                if !is_member {
                    return Err(AuthorizationError::Unauthorized);
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::{AuthorizationService, AuthorizationServiceImpl};
    use crate::{
        dto::{RepositoryAuthorizationRequest, RepositoryPermission},
        error::AuthorizationError,
        model::{Repository, RepositoryOwnerType, RepositoryVisibility},
        service::test_common::{
            MockOrganizationRepo, MockQuestionRepo, MockRepositoryRepo, MockReviewRepo,
            MockUserRepo,
        },
    };

    fn create_repository(
        owner_id: Uuid,
        owner_type: RepositoryOwnerType,
        visibility: RepositoryVisibility,
    ) -> Repository {
        Repository {
            id: Uuid::new_v4(),
            name: "myrepo".to_string(),
            owner_id,
            owner_name: "owner".to_string(),
            owner_type,
            visibility,
            description: None,
            stars: 0,
            user_star: false,
            readonly: false,
            created_at: chrono::Utc::now(),
        }
    }

    fn create_service(
        org_repo: MockOrganizationRepo,
        repo_repo: MockRepositoryRepo,
    ) -> AuthorizationServiceImpl<
        MockOrganizationRepo,
        MockRepositoryRepo,
        MockQuestionRepo,
        MockUserRepo,
        MockReviewRepo,
    > {
        AuthorizationServiceImpl {
            org_repo,
            repo_repo,
            question_repo: MockQuestionRepo::new(),
            user_repo: MockUserRepo::new(),
            review_repo: MockReviewRepo::new(),
        }
    }

    fn create_repo_auth_request(
        user_id: Option<Uuid>,
        permission: RepositoryPermission,
    ) -> RepositoryAuthorizationRequest {
        RepositoryAuthorizationRequest::new(user_id, "owner", "myrepo", permission).unwrap()
    }

    #[tokio::test]
    async fn repo_not_found() {
        let mut repo_repo = MockRepositoryRepo::new();
        repo_repo.expect_get().returning(|_, _, _| Ok(None));

        let service = create_service(MockOrganizationRepo::new(), repo_repo);
        let request = create_repo_auth_request(None, RepositoryPermission::Read);
        let err = service
            .verify_authorized_for_repository(request)
            .await
            .unwrap_err();

        assert!(matches!(err, AuthorizationError::NotFound(_)));
    }

    #[tokio::test]
    async fn read_public_repo_allows_anonymous() {
        let mut repo_repo = MockRepositoryRepo::new();
        let owner_id = Uuid::new_v4();
        repo_repo.expect_get().returning(move |_, _, _| {
            Ok(Some(create_repository(
                owner_id,
                RepositoryOwnerType::User,
                RepositoryVisibility::Public,
            )))
        });

        let service = create_service(MockOrganizationRepo::new(), repo_repo);
        let request = create_repo_auth_request(None, RepositoryPermission::Read);
        service
            .verify_authorized_for_repository(request)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn read_public_repo_allows_authenticated() {
        let mut repo_repo = MockRepositoryRepo::new();
        let owner_id = Uuid::new_v4();
        repo_repo.expect_get().returning(move |_, _, _| {
            Ok(Some(create_repository(
                owner_id,
                RepositoryOwnerType::User,
                RepositoryVisibility::Public,
            )))
        });

        let service = create_service(MockOrganizationRepo::new(), repo_repo);
        let request = create_repo_auth_request(Some(Uuid::new_v4()), RepositoryPermission::Read);
        service
            .verify_authorized_for_repository(request)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn read_private_user_repo_by_owner() {
        let owner_id = Uuid::new_v4();
        let mut repo_repo = MockRepositoryRepo::new();
        repo_repo.expect_get().returning(move |_, _, _| {
            Ok(Some(create_repository(
                owner_id,
                RepositoryOwnerType::User,
                RepositoryVisibility::Private,
            )))
        });

        let service = create_service(MockOrganizationRepo::new(), repo_repo);
        let request = create_repo_auth_request(Some(owner_id), RepositoryPermission::Read);
        service
            .verify_authorized_for_repository(request)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn read_private_user_repo_by_other() {
        let owner_id = Uuid::new_v4();
        let mut repo_repo = MockRepositoryRepo::new();
        repo_repo.expect_get().returning(move |_, _, _| {
            Ok(Some(create_repository(
                owner_id,
                RepositoryOwnerType::User,
                RepositoryVisibility::Private,
            )))
        });

        let service = create_service(MockOrganizationRepo::new(), repo_repo);
        let request = create_repo_auth_request(Some(Uuid::new_v4()), RepositoryPermission::Read);
        let err = service
            .verify_authorized_for_repository(request)
            .await
            .unwrap_err();

        assert!(matches!(err, AuthorizationError::Unauthorized));
    }

    #[tokio::test]
    async fn read_private_user_repo_anonymous() {
        let owner_id = Uuid::new_v4();
        let mut repo_repo = MockRepositoryRepo::new();
        repo_repo.expect_get().returning(move |_, _, _| {
            Ok(Some(create_repository(
                owner_id,
                RepositoryOwnerType::User,
                RepositoryVisibility::Private,
            )))
        });

        let service = create_service(MockOrganizationRepo::new(), repo_repo);
        let request = create_repo_auth_request(None, RepositoryPermission::Read);
        let err = service
            .verify_authorized_for_repository(request)
            .await
            .unwrap_err();

        assert!(matches!(err, AuthorizationError::Unauthorized));
    }

    #[tokio::test]
    async fn read_private_org_repo_by_member() {
        let owner_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let mut repo_repo = MockRepositoryRepo::new();
        repo_repo.expect_get().returning(move |_, _, _| {
            Ok(Some(create_repository(
                owner_id,
                RepositoryOwnerType::Organization,
                RepositoryVisibility::Private,
            )))
        });

        let mut org_repo = MockOrganizationRepo::new();
        org_repo
            .expect_is_member()
            .withf(move |oid, uid| *oid == owner_id && *uid == user_id)
            .returning(|_, _| Ok(true));

        let service = create_service(org_repo, repo_repo);
        let request = create_repo_auth_request(Some(user_id), RepositoryPermission::Read);
        service
            .verify_authorized_for_repository(request)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn read_private_org_repo_by_non_member() {
        let owner_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let mut repo_repo = MockRepositoryRepo::new();
        repo_repo.expect_get().returning(move |_, _, _| {
            Ok(Some(create_repository(
                owner_id,
                RepositoryOwnerType::Organization,
                RepositoryVisibility::Private,
            )))
        });

        let mut org_repo = MockOrganizationRepo::new();
        org_repo.expect_is_member().returning(|_, _| Ok(false));

        let service = create_service(org_repo, repo_repo);
        let request = create_repo_auth_request(Some(user_id), RepositoryPermission::Read);
        let err = service
            .verify_authorized_for_repository(request)
            .await
            .unwrap_err();

        assert!(matches!(err, AuthorizationError::Unauthorized));
    }

    #[tokio::test]
    async fn read_private_org_repo_anonymous() {
        let owner_id = Uuid::new_v4();
        let mut repo_repo = MockRepositoryRepo::new();
        repo_repo.expect_get().returning(move |_, _, _| {
            Ok(Some(create_repository(
                owner_id,
                RepositoryOwnerType::Organization,
                RepositoryVisibility::Private,
            )))
        });

        let service = create_service(MockOrganizationRepo::new(), repo_repo);
        let request = create_repo_auth_request(None, RepositoryPermission::Read);
        let err = service
            .verify_authorized_for_repository(request)
            .await
            .unwrap_err();

        assert!(matches!(err, AuthorizationError::Unauthorized));
    }

    #[tokio::test]
    async fn write_user_repo_by_owner() {
        let owner_id = Uuid::new_v4();
        let mut repo_repo = MockRepositoryRepo::new();
        repo_repo.expect_get().returning(move |_, _, _| {
            Ok(Some(create_repository(
                owner_id,
                RepositoryOwnerType::User,
                RepositoryVisibility::Public,
            )))
        });

        let service = create_service(MockOrganizationRepo::new(), repo_repo);
        let request = create_repo_auth_request(Some(owner_id), RepositoryPermission::Write);
        service
            .verify_authorized_for_repository(request)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn write_user_repo_by_other() {
        let owner_id = Uuid::new_v4();
        let mut repo_repo = MockRepositoryRepo::new();
        repo_repo.expect_get().returning(move |_, _, _| {
            Ok(Some(create_repository(
                owner_id,
                RepositoryOwnerType::User,
                RepositoryVisibility::Public,
            )))
        });

        let service = create_service(MockOrganizationRepo::new(), repo_repo);
        let request = create_repo_auth_request(Some(Uuid::new_v4()), RepositoryPermission::Write);
        let err = service
            .verify_authorized_for_repository(request)
            .await
            .unwrap_err();

        assert!(matches!(err, AuthorizationError::Unauthorized));
    }

    #[tokio::test]
    async fn write_user_repo_anonymous() {
        let owner_id = Uuid::new_v4();
        let mut repo_repo = MockRepositoryRepo::new();
        repo_repo.expect_get().returning(move |_, _, _| {
            Ok(Some(create_repository(
                owner_id,
                RepositoryOwnerType::User,
                RepositoryVisibility::Public,
            )))
        });

        let service = create_service(MockOrganizationRepo::new(), repo_repo);
        let request = create_repo_auth_request(None, RepositoryPermission::Write);
        let err = service
            .verify_authorized_for_repository(request)
            .await
            .unwrap_err();

        assert!(matches!(err, AuthorizationError::Unauthorized));
    }

    #[tokio::test]
    async fn write_org_repo_by_member() {
        let owner_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let mut repo_repo = MockRepositoryRepo::new();
        repo_repo.expect_get().returning(move |_, _, _| {
            Ok(Some(create_repository(
                owner_id,
                RepositoryOwnerType::Organization,
                RepositoryVisibility::Public,
            )))
        });

        let mut org_repo = MockOrganizationRepo::new();
        org_repo
            .expect_is_member()
            .withf(move |oid, uid| *oid == owner_id && *uid == user_id)
            .returning(|_, _| Ok(true));

        let service = create_service(org_repo, repo_repo);
        let request = create_repo_auth_request(Some(user_id), RepositoryPermission::Write);
        service
            .verify_authorized_for_repository(request)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn write_org_repo_by_non_member() {
        let owner_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let mut repo_repo = MockRepositoryRepo::new();
        repo_repo.expect_get().returning(move |_, _, _| {
            Ok(Some(create_repository(
                owner_id,
                RepositoryOwnerType::Organization,
                RepositoryVisibility::Public,
            )))
        });

        let mut org_repo = MockOrganizationRepo::new();
        org_repo.expect_is_member().returning(|_, _| Ok(false));

        let service = create_service(org_repo, repo_repo);
        let request = create_repo_auth_request(Some(user_id), RepositoryPermission::Write);
        let err = service
            .verify_authorized_for_repository(request)
            .await
            .unwrap_err();

        assert!(matches!(err, AuthorizationError::Unauthorized));
    }

    #[tokio::test]
    async fn write_org_repo_anonymous() {
        let owner_id = Uuid::new_v4();
        let mut repo_repo = MockRepositoryRepo::new();
        repo_repo.expect_get().returning(move |_, _, _| {
            Ok(Some(create_repository(
                owner_id,
                RepositoryOwnerType::Organization,
                RepositoryVisibility::Public,
            )))
        });

        let service = create_service(MockOrganizationRepo::new(), repo_repo);
        let request = create_repo_auth_request(None, RepositoryPermission::Write);
        let err = service
            .verify_authorized_for_repository(request)
            .await
            .unwrap_err();

        assert!(matches!(err, AuthorizationError::Unauthorized));
    }

    #[tokio::test]
    async fn admin_user_repo_by_owner() {
        let owner_id = Uuid::new_v4();
        let mut repo_repo = MockRepositoryRepo::new();
        repo_repo.expect_get().returning(move |_, _, _| {
            Ok(Some(create_repository(
                owner_id,
                RepositoryOwnerType::User,
                RepositoryVisibility::Public,
            )))
        });

        let service = create_service(MockOrganizationRepo::new(), repo_repo);
        let request = create_repo_auth_request(Some(owner_id), RepositoryPermission::Admin);
        service
            .verify_authorized_for_repository(request)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn admin_user_repo_by_other() {
        let owner_id = Uuid::new_v4();
        let mut repo_repo = MockRepositoryRepo::new();
        repo_repo.expect_get().returning(move |_, _, _| {
            Ok(Some(create_repository(
                owner_id,
                RepositoryOwnerType::User,
                RepositoryVisibility::Public,
            )))
        });

        let service = create_service(MockOrganizationRepo::new(), repo_repo);
        let request = create_repo_auth_request(Some(Uuid::new_v4()), RepositoryPermission::Admin);
        let err = service
            .verify_authorized_for_repository(request)
            .await
            .unwrap_err();

        assert!(matches!(err, AuthorizationError::Unauthorized));
    }

    #[tokio::test]
    async fn admin_user_repo_anonymous() {
        let owner_id = Uuid::new_v4();
        let mut repo_repo = MockRepositoryRepo::new();
        repo_repo.expect_get().returning(move |_, _, _| {
            Ok(Some(create_repository(
                owner_id,
                RepositoryOwnerType::User,
                RepositoryVisibility::Public,
            )))
        });

        let service = create_service(MockOrganizationRepo::new(), repo_repo);
        let request = create_repo_auth_request(None, RepositoryPermission::Admin);
        let err = service
            .verify_authorized_for_repository(request)
            .await
            .unwrap_err();

        assert!(matches!(err, AuthorizationError::Unauthorized));
    }

    #[tokio::test]
    async fn admin_org_repo_by_admin() {
        let owner_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let mut repo_repo = MockRepositoryRepo::new();
        repo_repo.expect_get().returning(move |_, _, _| {
            Ok(Some(create_repository(
                owner_id,
                RepositoryOwnerType::Organization,
                RepositoryVisibility::Public,
            )))
        });

        let mut org_repo = MockOrganizationRepo::new();
        org_repo
            .expect_get_member_role()
            .withf(move |name, uid| name == "owner" && *uid == user_id)
            .returning(|_, _| Ok(Some(crate::model::OrganizationRole::Admin)));

        let service = create_service(org_repo, repo_repo);
        let request = create_repo_auth_request(Some(user_id), RepositoryPermission::Admin);
        service
            .verify_authorized_for_repository(request)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn admin_org_repo_by_member() {
        let owner_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let mut repo_repo = MockRepositoryRepo::new();
        repo_repo.expect_get().returning(move |_, _, _| {
            Ok(Some(create_repository(
                owner_id,
                RepositoryOwnerType::Organization,
                RepositoryVisibility::Public,
            )))
        });

        let mut org_repo = MockOrganizationRepo::new();
        org_repo
            .expect_get_member_role()
            .returning(|_, _| Ok(Some(crate::model::OrganizationRole::Member)));

        let service = create_service(org_repo, repo_repo);
        let request = create_repo_auth_request(Some(user_id), RepositoryPermission::Admin);
        let err = service
            .verify_authorized_for_repository(request)
            .await
            .unwrap_err();

        assert!(matches!(err, AuthorizationError::Unauthorized));
    }

    #[tokio::test]
    async fn admin_org_repo_by_non_member() {
        let owner_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let mut repo_repo = MockRepositoryRepo::new();
        repo_repo.expect_get().returning(move |_, _, _| {
            Ok(Some(create_repository(
                owner_id,
                RepositoryOwnerType::Organization,
                RepositoryVisibility::Public,
            )))
        });

        let mut org_repo = MockOrganizationRepo::new();
        org_repo.expect_get_member_role().returning(|_, _| Ok(None));

        let service = create_service(org_repo, repo_repo);
        let request = create_repo_auth_request(Some(user_id), RepositoryPermission::Admin);
        let err = service
            .verify_authorized_for_repository(request)
            .await
            .unwrap_err();

        assert!(matches!(err, AuthorizationError::Unauthorized));
    }

    #[tokio::test]
    async fn admin_org_repo_anonymous() {
        let owner_id = Uuid::new_v4();
        let mut repo_repo = MockRepositoryRepo::new();
        repo_repo.expect_get().returning(move |_, _, _| {
            Ok(Some(create_repository(
                owner_id,
                RepositoryOwnerType::Organization,
                RepositoryVisibility::Public,
            )))
        });

        let service = create_service(MockOrganizationRepo::new(), repo_repo);
        let request = create_repo_auth_request(None, RepositoryPermission::Admin);
        let err = service
            .verify_authorized_for_repository(request)
            .await
            .unwrap_err();

        assert!(matches!(err, AuthorizationError::Unauthorized));
    }

    #[tokio::test]
    async fn repo_get_db_error() {
        let mut repo_repo = MockRepositoryRepo::new();
        repo_repo
            .expect_get()
            .returning(|_, _, _| Err(crate::error::DatabaseError::RowNotFound));

        let service = create_service(MockOrganizationRepo::new(), repo_repo);
        let request = create_repo_auth_request(None, RepositoryPermission::Read);
        let err = service
            .verify_authorized_for_repository(request)
            .await
            .unwrap_err();

        assert!(matches!(err, AuthorizationError::DatabaseError(_)));
    }

    #[tokio::test]
    async fn org_is_member_db_error() {
        let owner_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let mut repo_repo = MockRepositoryRepo::new();
        repo_repo.expect_get().returning(move |_, _, _| {
            Ok(Some(create_repository(
                owner_id,
                RepositoryOwnerType::Organization,
                RepositoryVisibility::Private,
            )))
        });

        let mut org_repo = MockOrganizationRepo::new();
        org_repo
            .expect_is_member()
            .returning(|_, _| Err(crate::error::DatabaseError::RowNotFound));

        let service = create_service(org_repo, repo_repo);
        let request = create_repo_auth_request(Some(user_id), RepositoryPermission::Read);
        let err = service
            .verify_authorized_for_repository(request)
            .await
            .unwrap_err();

        assert!(matches!(err, AuthorizationError::DatabaseError(_)));
    }

    #[tokio::test]
    async fn org_get_member_role_db_error() {
        let owner_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let mut repo_repo = MockRepositoryRepo::new();
        repo_repo.expect_get().returning(move |_, _, _| {
            Ok(Some(create_repository(
                owner_id,
                RepositoryOwnerType::Organization,
                RepositoryVisibility::Public,
            )))
        });

        let mut org_repo = MockOrganizationRepo::new();
        org_repo
            .expect_get_member_role()
            .returning(|_, _| Err(crate::error::DatabaseError::RowNotFound));

        let service = create_service(org_repo, repo_repo);
        let request = create_repo_auth_request(Some(user_id), RepositoryPermission::Admin);
        let err = service
            .verify_authorized_for_repository(request)
            .await
            .unwrap_err();

        assert!(matches!(err, AuthorizationError::DatabaseError(_)));
    }

    #[tokio::test]
    async fn write_readonly_repo_by_owner() {
        let owner_id = Uuid::new_v4();
        let mut repo_repo = MockRepositoryRepo::new();
        repo_repo.expect_get().returning(move |_, _, _| {
            let mut repo = create_repository(
                owner_id,
                RepositoryOwnerType::User,
                RepositoryVisibility::Public,
            );
            repo.readonly = true;
            Ok(Some(repo))
        });

        let service = create_service(MockOrganizationRepo::new(), repo_repo);
        let request = create_repo_auth_request(Some(owner_id), RepositoryPermission::Write);
        let err = service
            .verify_authorized_for_repository(request)
            .await
            .unwrap_err();

        assert!(matches!(err, AuthorizationError::ReadonlyRepository));
    }

    #[tokio::test]
    async fn admin_readonly_repo_by_owner() {
        let owner_id = Uuid::new_v4();
        let mut repo_repo = MockRepositoryRepo::new();
        repo_repo.expect_get().returning(move |_, _, _| {
            let mut repo = create_repository(
                owner_id,
                RepositoryOwnerType::User,
                RepositoryVisibility::Public,
            );
            repo.readonly = true;
            Ok(Some(repo))
        });

        let service = create_service(MockOrganizationRepo::new(), repo_repo);
        let request = create_repo_auth_request(Some(owner_id), RepositoryPermission::Admin);

        // Admin ops are intentionally permitted on readonly repos; only Write is blocked.
        let result = service.verify_authorized_for_repository(request).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn read_readonly_public_repo_anonymous() {
        let owner_id = Uuid::new_v4();
        let mut repo_repo = MockRepositoryRepo::new();
        repo_repo.expect_get().returning(move |_, _, _| {
            let mut repo = create_repository(
                owner_id,
                RepositoryOwnerType::User,
                RepositoryVisibility::Public,
            );
            repo.readonly = true;
            Ok(Some(repo))
        });

        let service = create_service(MockOrganizationRepo::new(), repo_repo);
        let request = create_repo_auth_request(None, RepositoryPermission::Read);
        service
            .verify_authorized_for_repository(request)
            .await
            .unwrap();
    }
}
