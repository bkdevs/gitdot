use async_trait::async_trait;

use crate::{
    dto::{
        AnswerAuthorizationRequest, CommentAuthorizationRequest, OrganizationAuthorizationRequest,
        QuestionAuthorizationRequest, RepositoryAuthorizationRequest,
        RepositoryCreationAuthorizationRequest, RepositoryPermission, ValidateTokenRequest,
        ValidateTokenResponse,
    },
    error::AuthorizationError,
    model::{OrganizationRole, RepositoryOwnerType},
    repository::{
        OrganizationRepository, OrganizationRepositoryImpl, QuestionRepository,
        QuestionRepositoryImpl, RepositoryRepository, RepositoryRepositoryImpl, TokenRepository,
        TokenRepositoryImpl, UserRepository, UserRepositoryImpl,
    },
    util::token::{hash_token, validate_token_format},
};

#[async_trait]
pub trait AuthorizationService: Send + Sync + 'static {
    async fn validate_token(
        &self,
        request: ValidateTokenRequest,
    ) -> Result<ValidateTokenResponse, AuthorizationError>;

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
}

#[derive(Debug, Clone)]
pub struct AuthorizationServiceImpl<T, O, R, Q, U>
where
    T: TokenRepository,
    O: OrganizationRepository,
    R: RepositoryRepository,
    Q: QuestionRepository,
    U: UserRepository,
{
    token_repo: T,
    org_repo: O,
    repo_repo: R,
    question_repo: Q,
    user_repo: U,
}

impl
    AuthorizationServiceImpl<
        TokenRepositoryImpl,
        OrganizationRepositoryImpl,
        RepositoryRepositoryImpl,
        QuestionRepositoryImpl,
        UserRepositoryImpl,
    >
{
    pub fn new(
        token_repo: TokenRepositoryImpl,
        org_repo: OrganizationRepositoryImpl,
        repo_repo: RepositoryRepositoryImpl,
        question_repo: QuestionRepositoryImpl,
        user_repo: UserRepositoryImpl,
    ) -> Self {
        Self {
            token_repo,
            org_repo,
            repo_repo,
            question_repo,
            user_repo,
        }
    }
}

#[async_trait]
impl<T, O, R, Q, U> AuthorizationService for AuthorizationServiceImpl<T, O, R, Q, U>
where
    T: TokenRepository,
    O: OrganizationRepository,
    R: RepositoryRepository,
    Q: QuestionRepository,
    U: UserRepository,
{
    async fn validate_token(
        &self,
        request: ValidateTokenRequest,
    ) -> Result<ValidateTokenResponse, AuthorizationError> {
        if !validate_token_format(&request.token) {
            return Err(AuthorizationError::Unauthorized);
        }
        if !&request.token.starts_with(request.token_type.prefix()) {
            return Err(AuthorizationError::Unauthorized);
        }

        let token_hash = hash_token(&request.token);
        let access_token = self
            .token_repo
            .get_token_by_hash(&token_hash)
            .await?
            .ok_or(AuthorizationError::Unauthorized)?;

        self.token_repo.touch_token(access_token.id).await?;

        Ok(ValidateTokenResponse {
            principal_id: access_token.principal_id,
        })
    }

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
                    .ok_or_else(|| {
                        AuthorizationError::NotFound(format!(
                            "Organization not found: {}",
                            request.owner.as_ref()
                        ))
                    })?;

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
            .get(request.owner.as_ref(), request.repo.as_ref())
            .await?
            .ok_or_else(|| {
                AuthorizationError::NotFound(format!(
                    "Repository not found: {}",
                    request.get_repo_path(),
                ))
            })?;

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
            .ok_or_else(|| {
                AuthorizationError::NotFound(format!(
                    "Question not found: {}",
                    request.get_repo_path(),
                ))
            })?;

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
            .ok_or_else(|| {
                AuthorizationError::NotFound(format!("Answer not found: {}", request.answer_id))
            })?;

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
            .ok_or_else(|| {
                AuthorizationError::NotFound(format!("Comment not found: {}", request.comment_id))
            })?;

        if author_id != request.user_id {
            return Err(AuthorizationError::Unauthorized);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use mockall::mock;
    use uuid::Uuid;

    use crate::{
        dto::{RepositoryAuthorizationRequest, RepositoryPermission},
        error::AuthorizationError,
        model::{
            AccessToken, Answer, Comment, Organization, OrganizationMember, OrganizationRole,
            Question, Repository, RepositoryOwnerType, RepositoryVisibility, TokenType, User,
            VoteResult, VoteTarget,
        },
        repository::{
            OrganizationRepository, QuestionRepository, RepositoryRepository, TokenRepository,
            UserRepository,
        },
    };

    use super::{AuthorizationService, AuthorizationServiceImpl};

    mock! {
        pub TokenRepo {}
        impl Clone for TokenRepo {
            fn clone(&self) -> Self;
        }
        #[async_trait]
        impl TokenRepository for TokenRepo {
            async fn create_token(&self, principal_id: Uuid, client_id: &str, token_hash: &str, token_type: TokenType) -> Result<AccessToken, sqlx::Error>;
            async fn get_token_by_hash(&self, token_hash: &str) -> Result<Option<AccessToken>, sqlx::Error>;
            async fn touch_token(&self, id: Uuid) -> Result<(), sqlx::Error>;
            async fn delete_token(&self, id: Uuid) -> Result<(), sqlx::Error>;
            async fn delete_token_by_principal(&self, runner_id: Uuid) -> Result<(), sqlx::Error>;
        }
    }

    mock! {
        pub OrganizationRepo {}
        impl Clone for OrganizationRepo {
            fn clone(&self) -> Self;
        }
        #[async_trait]
        impl OrganizationRepository for OrganizationRepo {
            async fn create(&self, org_name: &str, owner_id: Uuid) -> Result<Organization, sqlx::Error>;
            async fn get(&self, org_name: &str) -> Result<Option<Organization>, sqlx::Error>;
            async fn is_member(&self, org_id: Uuid, user_id: Uuid) -> Result<bool, sqlx::Error>;
            async fn add_member(&self, org_name: &str, user_name: &str, role: OrganizationRole) -> Result<Option<OrganizationMember>, sqlx::Error>;
            async fn get_member_role(&self, org_name: &str, user_id: Uuid) -> Result<Option<OrganizationRole>, sqlx::Error>;
            async fn list(&self) -> Result<Vec<Organization>, sqlx::Error>;
            async fn list_by_user_id(&self, user_id: Uuid) -> Result<Vec<Organization>, sqlx::Error>;
        }
    }

    mock! {
        pub RepositoryRepo {}
        impl Clone for RepositoryRepo {
            fn clone(&self) -> Self;
        }
        #[async_trait]
        impl RepositoryRepository for RepositoryRepo {
            async fn create(&self, name: &str, owner_id: Uuid, owner_name: &str, owner_type: &RepositoryOwnerType, visibility: &RepositoryVisibility) -> Result<Repository, sqlx::Error>;
            async fn get(&self, owner: &str, repo: &str) -> Result<Option<Repository>, sqlx::Error>;
            async fn get_by_id(&self, id: Uuid) -> Result<Option<Repository>, sqlx::Error>;
            async fn list_by_owner(&self, owner_name: &str) -> Result<Vec<Repository>, sqlx::Error>;
            async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error>;
        }
    }

    mock! {
        pub QuestionRepo {}
        impl Clone for QuestionRepo {
            fn clone(&self) -> Self;
        }
        #[async_trait]
        impl QuestionRepository for QuestionRepo {
            async fn create_question(&self, author_id: Uuid, repository_id: Uuid, title: &str, body: &str) -> Result<Question, sqlx::Error>;
            async fn update_question(&self, repository_id: Uuid, number: i32, title: &str, body: &str) -> Result<Option<Question>, sqlx::Error>;
            async fn get_question(&self, repository_id: Uuid, number: i32, user_id: Option<Uuid>) -> Result<Option<Question>, sqlx::Error>;
            async fn get_question_id(&self, owner: &str, repo: &str, question_number: i32) -> Result<Option<Uuid>, sqlx::Error>;
            async fn get_questions(&self, repository_id: Uuid, user_id: Option<Uuid>) -> Result<Vec<Question>, sqlx::Error>;
            async fn create_answer(&self, owner: &str, repo: &str, question_number: i32, author_id: Uuid, body: &str) -> Result<Option<Answer>, sqlx::Error>;
            async fn update_answer(&self, id: Uuid, body: &str) -> Result<Option<Answer>, sqlx::Error>;
            async fn create_comment(&self, parent_id: Uuid, author_id: Uuid, body: &str) -> Result<Comment, sqlx::Error>;
            async fn create_question_comment(&self, owner: &str, repo: &str, question_number: i32, author_id: Uuid, body: &str) -> Result<Option<Comment>, sqlx::Error>;
            async fn update_comment(&self, id: Uuid, body: &str) -> Result<Option<Comment>, sqlx::Error>;
            async fn get_question_author_id(&self, owner: &str, repo: &str, question_number: i32) -> Result<Option<Uuid>, sqlx::Error>;
            async fn get_answer_author_id(&self, id: Uuid) -> Result<Option<Uuid>, sqlx::Error>;
            async fn get_comment_author_id(&self, id: Uuid) -> Result<Option<Uuid>, sqlx::Error>;
            async fn vote(&self, user_id: Uuid, target_id: Uuid, target_type: VoteTarget, value: i16) -> Result<VoteResult, sqlx::Error>;
        }
    }

    mock! {
        pub UserRepo {}
        impl Clone for UserRepo {
            fn clone(&self) -> Self;
        }
        #[async_trait]
        impl UserRepository for UserRepo {
            async fn get(&self, user_name: &str) -> Result<Option<User>, sqlx::Error>;
            async fn update(&self, id: Uuid, name: &str) -> Result<User, sqlx::Error>;
            async fn get_by_id(&self, id: Uuid) -> Result<Option<User>, sqlx::Error>;
            async fn get_by_emails(&self, emails: &[String]) -> Result<Vec<User>, sqlx::Error>;
            async fn is_name_taken(&self, name: &str) -> Result<bool, sqlx::Error>;
            async fn is_email_taken(&self, email: &str) -> Result<bool, sqlx::Error>;
        }
    }

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
            created_at: chrono::Utc::now(),
        }
    }

    fn create_service(
        org_repo: MockOrganizationRepo,
        repo_repo: MockRepositoryRepo,
    ) -> AuthorizationServiceImpl<
        MockTokenRepo,
        MockOrganizationRepo,
        MockRepositoryRepo,
        MockQuestionRepo,
        MockUserRepo,
    > {
        AuthorizationServiceImpl {
            token_repo: MockTokenRepo::new(),
            org_repo,
            repo_repo,
            question_repo: MockQuestionRepo::new(),
            user_repo: MockUserRepo::new(),
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
        repo_repo.expect_get().returning(|_, _| Ok(None));

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
        repo_repo.expect_get().returning(move |_, _| {
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
        repo_repo.expect_get().returning(move |_, _| {
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
        repo_repo.expect_get().returning(move |_, _| {
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
        repo_repo.expect_get().returning(move |_, _| {
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
        repo_repo.expect_get().returning(move |_, _| {
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
        repo_repo.expect_get().returning(move |_, _| {
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
        repo_repo.expect_get().returning(move |_, _| {
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
        repo_repo.expect_get().returning(move |_, _| {
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
        repo_repo.expect_get().returning(move |_, _| {
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
        repo_repo.expect_get().returning(move |_, _| {
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
        repo_repo.expect_get().returning(move |_, _| {
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
        repo_repo.expect_get().returning(move |_, _| {
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
        repo_repo.expect_get().returning(move |_, _| {
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
        repo_repo.expect_get().returning(move |_, _| {
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
        repo_repo.expect_get().returning(move |_, _| {
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
        repo_repo.expect_get().returning(move |_, _| {
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
        repo_repo.expect_get().returning(move |_, _| {
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
        repo_repo.expect_get().returning(move |_, _| {
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
        repo_repo.expect_get().returning(move |_, _| {
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
        repo_repo.expect_get().returning(move |_, _| {
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
        repo_repo.expect_get().returning(move |_, _| {
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
            .returning(|_, _| Err(sqlx::Error::RowNotFound));

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
        repo_repo.expect_get().returning(move |_, _| {
            Ok(Some(create_repository(
                owner_id,
                RepositoryOwnerType::Organization,
                RepositoryVisibility::Private,
            )))
        });

        let mut org_repo = MockOrganizationRepo::new();
        org_repo
            .expect_is_member()
            .returning(|_, _| Err(sqlx::Error::RowNotFound));

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
        repo_repo.expect_get().returning(move |_, _| {
            Ok(Some(create_repository(
                owner_id,
                RepositoryOwnerType::Organization,
                RepositoryVisibility::Public,
            )))
        });

        let mut org_repo = MockOrganizationRepo::new();
        org_repo
            .expect_get_member_role()
            .returning(|_, _| Err(sqlx::Error::RowNotFound));

        let service = create_service(org_repo, repo_repo);
        let request = create_repo_auth_request(Some(user_id), RepositoryPermission::Admin);
        let err = service
            .verify_authorized_for_repository(request)
            .await
            .unwrap_err();

        assert!(matches!(err, AuthorizationError::DatabaseError(_)));
    }
}
