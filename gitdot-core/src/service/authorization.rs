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
    use async_trait::async_trait;
    use chrono::{DateTime, Utc};
    use mockall::mock;
    use uuid::Uuid;

    use crate::{
        dto::{RepositoryAuthorizationRequest, RepositoryPermission},
        error::AuthorizationError,
        model::{
            Answer, AuthProvider, Comment, CommentSide, Diff, DiffStatus, Organization,
            OrganizationMember, OrganizationRole, Question, Repository, RepositoryOwnerType,
            RepositoryStar, RepositoryVisibility, Review, ReviewComment, ReviewStatus, Reviewer,
            Revision, User, UserEmail, UserOrganization, Verdict, VoteResult, VoteTarget,
        },
        repository::{
            OrganizationRepository, QuestionRepository, RepositoryRepository, ReviewRepository,
            UserRepository,
        },
    };

    use super::{AuthorizationService, AuthorizationServiceImpl};

    mock! {
        pub OrganizationRepo {}
        impl Clone for OrganizationRepo {
            fn clone(&self) -> Self;
        }
        #[async_trait]
        impl OrganizationRepository for OrganizationRepo {
            async fn create(&self, org_name: &str, owner_id: Uuid, readme: Option<String>) -> Result<Organization, crate::error::DatabaseError>;
            async fn get(&self, org_name: &str) -> Result<Option<Organization>, crate::error::DatabaseError>;
            async fn touch_image(&self, org_id: Uuid) -> Result<(), crate::error::DatabaseError>;
            async fn is_member(&self, org_id: Uuid, user_id: Uuid) -> Result<bool, crate::error::DatabaseError>;
            async fn add_member(&self, org_name: &str, user_name: &str, role: OrganizationRole, role_description: Option<String>) -> Result<Option<OrganizationMember>, crate::error::DatabaseError>;
            async fn get_member_role(&self, org_name: &str, user_id: Uuid) -> Result<Option<OrganizationRole>, crate::error::DatabaseError>;
            async fn get_member(&self, org_name: &str, member_id: Uuid) -> Result<Option<OrganizationMember>, crate::error::DatabaseError>;
            async fn update(&self, org_name: &str, location: Option<String>, readme: Option<String>, links: Option<Vec<String>>, display_name: Option<String>) -> Result<Option<Organization>, crate::error::DatabaseError>;
            async fn update_member(&self, org_name: &str, member_id: Uuid, role_description: Option<String>) -> Result<Option<OrganizationMember>, crate::error::DatabaseError>;
            async fn list(&self, cursor: Option<crate::dto::Cursor>, limit: i64) -> Result<(Vec<Organization>, Option<crate::dto::Cursor>), crate::error::DatabaseError>;
            async fn list_by_user_id(&self, user_id: Uuid) -> Result<Vec<Organization>, crate::error::DatabaseError>;
            async fn list_memberships_by_user_id(&self, user_id: Uuid, cursor: Option<crate::dto::Cursor>, limit: i64) -> Result<(Vec<UserOrganization>, Option<crate::dto::Cursor>), crate::error::DatabaseError>;
        }
    }

    mock! {
        pub RepositoryRepo {}
        impl Clone for RepositoryRepo {
            fn clone(&self) -> Self;
        }
        #[async_trait]
        impl RepositoryRepository for RepositoryRepo {
            async fn create(&self, name: &str, owner_id: Uuid, owner_type: &RepositoryOwnerType, visibility: &RepositoryVisibility, description: Option<String>, readonly: bool, created_at: Option<chrono::DateTime<chrono::Utc>>) -> Result<Repository, crate::error::DatabaseError>;
            async fn get(&self, owner: &str, repo: &str, viewer_id: Option<Uuid>) -> Result<Option<Repository>, crate::error::DatabaseError>;
            async fn get_by_id(&self, id: Uuid, viewer_id: Option<Uuid>) -> Result<Option<Repository>, crate::error::DatabaseError>;
            async fn list_by_owner(&self, owner_name: &str, viewer_id: Option<Uuid>, cursor: Option<crate::dto::Cursor>, limit: i64) -> Result<(Vec<Repository>, Option<crate::dto::Cursor>), crate::error::DatabaseError>;
            async fn list_latest(&self, limit: i64) -> Result<Vec<Repository>, crate::error::DatabaseError>;
            async fn list_trending(&self, limit: i64) -> Result<Vec<Repository>, crate::error::DatabaseError>;
            async fn delete(&self, id: Uuid) -> Result<(), crate::error::DatabaseError>;
            async fn update(&self, id: Uuid, description: Option<String>) -> Result<Option<Repository>, crate::error::DatabaseError>;
            async fn disable_readonly(&self, owner: &str, repo: &str) -> Result<Option<Repository>, crate::error::DatabaseError>;
            async fn star(&self, id: Uuid, user_id: Uuid) -> Result<Option<RepositoryStar>, crate::error::DatabaseError>;
            async fn unstar(&self, id: Uuid, user_id: Uuid) -> Result<bool, crate::error::DatabaseError>;
            async fn list_recent_stars(&self, repository_id: Uuid, limit: i64) -> Result<Vec<(User, DateTime<Utc>)>, crate::error::DatabaseError>;
            async fn list_commit_filters(&self, repository_id: Uuid, cursor: Option<crate::dto::Cursor>, limit: i64) -> Result<(Vec<crate::model::CommitFilter>, Option<crate::dto::Cursor>), crate::error::DatabaseError>;
            async fn create_commit_filter(&self, repository_id: Uuid, name: &str, authors: Option<Vec<String>>, tags: Option<Vec<String>>, paths: Option<Vec<String>>) -> Result<crate::model::CommitFilter, crate::error::DatabaseError>;
            async fn update_commit_filter(&self, filter_id: Uuid, name: &str, authors: Option<Vec<String>>, tags: Option<Vec<String>>, paths: Option<Vec<String>>) -> Result<Option<crate::model::CommitFilter>, crate::error::DatabaseError>;
            async fn delete_commit_filter(&self, filter_id: Uuid) -> Result<bool, crate::error::DatabaseError>;
        }
    }

    mock! {
        pub QuestionRepo {}
        impl Clone for QuestionRepo {
            fn clone(&self) -> Self;
        }
        #[async_trait]
        impl QuestionRepository for QuestionRepo {
            async fn create_question(&self, author_id: Uuid, repository_id: Uuid, title: &str, body: &str) -> Result<Question, crate::error::DatabaseError>;
            async fn update_question(&self, repository_id: Uuid, number: i32, title: &str, body: &str) -> Result<Option<Question>, crate::error::DatabaseError>;
            async fn get_question(&self, repository_id: Uuid, number: i32, user_id: Option<Uuid>) -> Result<Option<Question>, crate::error::DatabaseError>;
            async fn get_question_id(&self, owner: &str, repo: &str, question_number: i32) -> Result<Option<Uuid>, crate::error::DatabaseError>;
            async fn list_questions(&self, repository_id: Uuid, user_id: Option<Uuid>, cursor: Option<crate::dto::Cursor>, limit: i64) -> Result<(Vec<Question>, Option<crate::dto::Cursor>), crate::error::DatabaseError>;
            async fn create_answer(&self, owner: &str, repo: &str, question_number: i32, author_id: Uuid, body: &str) -> Result<Option<Answer>, crate::error::DatabaseError>;
            async fn update_answer(&self, id: Uuid, body: &str) -> Result<Option<Answer>, crate::error::DatabaseError>;
            async fn create_comment(&self, parent_id: Uuid, author_id: Uuid, body: &str) -> Result<Comment, crate::error::DatabaseError>;
            async fn create_question_comment(&self, owner: &str, repo: &str, question_number: i32, author_id: Uuid, body: &str) -> Result<Option<Comment>, crate::error::DatabaseError>;
            async fn update_comment(&self, id: Uuid, body: &str) -> Result<Option<Comment>, crate::error::DatabaseError>;
            async fn get_question_author_id(&self, owner: &str, repo: &str, question_number: i32) -> Result<Option<Uuid>, crate::error::DatabaseError>;
            async fn get_answer_author_id(&self, id: Uuid) -> Result<Option<Uuid>, crate::error::DatabaseError>;
            async fn get_comment_author_id(&self, id: Uuid) -> Result<Option<Uuid>, crate::error::DatabaseError>;
            async fn vote(&self, user_id: Uuid, target_id: Uuid, target_type: VoteTarget, value: i16) -> Result<VoteResult, crate::error::DatabaseError>;
        }
    }

    mock! {
        pub UserRepo {}
        impl Clone for UserRepo {
            fn clone(&self) -> Self;
        }
        #[async_trait]
        impl UserRepository for UserRepo {
            async fn create(&self, email: &str, is_email_verified: bool, provider: AuthProvider) -> Result<User, crate::error::DatabaseError>;
            async fn get(&self, user_name: &str) -> Result<Option<User>, crate::error::DatabaseError>;
            async fn update(&self, id: Uuid, name: Option<String>, location: Option<String>, readme: Option<String>, links: Option<Vec<String>>, display_name: Option<String>) -> Result<User, crate::error::DatabaseError>;
            async fn get_by_id(&self, id: Uuid) -> Result<Option<User>, crate::error::DatabaseError>;
            async fn touch_image(&self, id: Uuid) -> Result<(), crate::error::DatabaseError>;
            async fn get_by_email(&self, email: &str) -> Result<Option<User>, crate::error::DatabaseError>;
            async fn get_by_emails(&self, emails: &[String]) -> Result<Vec<(String, Uuid)>, crate::error::DatabaseError>;
            async fn verify_email(&self, id: Uuid) -> Result<(), crate::error::DatabaseError>;
            async fn is_name_taken(&self, name: &str) -> Result<bool, crate::error::DatabaseError>;
            async fn list_emails(&self, user_id: Uuid) -> Result<Vec<UserEmail>, crate::error::DatabaseError>;
            async fn create_email(&self, user_id: Uuid, email: &str) -> Result<UserEmail, crate::error::DatabaseError>;
            async fn get_email_for_user(&self, user_id: Uuid, email: &str) -> Result<Option<UserEmail>, crate::error::DatabaseError>;
            async fn upsert_verified_emails(&self, user_id: Uuid, emails: &[String]) -> Result<(), crate::error::DatabaseError>;
            async fn list_starred_repositories(&self, user_id: Uuid, viewer_id: Option<Uuid>, cursor: Option<crate::dto::Cursor>, limit: i64) -> Result<(Vec<Repository>, Option<crate::dto::Cursor>), crate::error::DatabaseError>;
        }
    }

    mock! {
        pub ReviewRepo {}
        impl Clone for ReviewRepo {
            fn clone(&self) -> Self;
        }
        #[async_trait]
        impl ReviewRepository for ReviewRepo {
            async fn get_review_by_number(&self, owner: &str, repo: &str, number: i32) -> Result<Option<Review>, crate::error::DatabaseError>;
            async fn list_reviews(&self, owner: &str, repo: &str, viewer_id: Option<Uuid>, cursor: Option<crate::dto::Cursor>, limit: i64) -> Result<(Vec<Review>, Option<crate::dto::Cursor>), crate::error::DatabaseError>;
            async fn list_reviews_by_user(&self, user_name: &str, viewer_id: Option<Uuid>, status: Option<String>, owner: Option<String>, repo: Option<String>, cursor: Option<crate::dto::Cursor>, limit: i64) -> Result<(Vec<Review>, Option<crate::dto::Cursor>), crate::error::DatabaseError>;
            async fn create_review(&self, repository_id: Uuid, author_id: Uuid, target_branch: &str) -> Result<Review, crate::error::DatabaseError>;
            async fn update_review(&self, review_id: Uuid, status: Option<ReviewStatus>, title: Option<String>, description: Option<String>) -> Result<(), crate::error::DatabaseError>;
            async fn create_diff(&self, review_id: Uuid, position: i32, message: &str) -> Result<Diff, crate::error::DatabaseError>;
            async fn update_diff(&self, diff_id: Uuid, status: Option<DiffStatus>, message: Option<String>) -> Result<(), crate::error::DatabaseError>;
            async fn create_revision(&self, diff_id: Uuid, number: i32, commit_hash: &str, parent_hash: &str) -> Result<Revision, crate::error::DatabaseError>;
            async fn update_revision_sha(&self, revision_id: Uuid, commit_hash: &str, parent_hash: &str) -> Result<(), crate::error::DatabaseError>;
            async fn add_reviewer(&self, review_id: Uuid, reviewer_id: Uuid) -> Result<Option<Reviewer>, crate::error::DatabaseError>;
            async fn remove_reviewer(&self, review_id: Uuid, reviewer_id: Uuid) -> Result<bool, crate::error::DatabaseError>;
            async fn create_verdict(&self, diff_id: Uuid, revision_id: Uuid, reviewer_id: Uuid, verdict: Verdict) -> Result<(), crate::error::DatabaseError>;
            async fn create_comment(&self, review_id: Uuid, diff_id: Uuid, revision_id: Uuid, author_id: Uuid, body: &str, parent_id: Option<Uuid>, file_path: Option<String>, line_number_start: Option<i32>, line_number_end: Option<i32>, start_character: Option<i32>, end_character: Option<i32>, side: Option<CommentSide>) -> Result<ReviewComment, crate::error::DatabaseError>;
            async fn get_comment(&self, comment_id: Uuid) -> Result<Option<ReviewComment>, crate::error::DatabaseError>;
            async fn update_comment(&self, comment_id: Uuid, body: &str) -> Result<ReviewComment, crate::error::DatabaseError>;
            async fn resolve_comment(&self, comment_id: Uuid, resolved: bool) -> Result<(), crate::error::DatabaseError>;
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
        let err = service
            .verify_authorized_for_repository(request)
            .await
            .unwrap_err();

        assert!(matches!(err, AuthorizationError::ReadonlyRepository));
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
