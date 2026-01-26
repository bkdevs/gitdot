use async_trait::async_trait;

use crate::dto::{
    AnswerAuthorizationRequest, CommentAuthorizationRequest, OrganizationAuthorizationRequest,
    QuestionAuthorizationRequest, RepositoryAuthorizationRequest,
};
use crate::error::AuthorizationError;
use crate::repository::{
    OrganizationRepository, OrganizationRepositoryImpl, QuestionRepository, QuestionRepositoryImpl,
    RepositoryRepository, RepositoryRepositoryImpl,
};

#[async_trait]
pub trait AuthorizationService: Send + Sync + 'static {
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
pub struct AuthorizationServiceImpl<O, R, Q>
where
    O: OrganizationRepository,
    R: RepositoryRepository,
    Q: QuestionRepository,
{
    org_repo: O,
    repo_repo: R,
    question_repo: Q,
}

impl
    AuthorizationServiceImpl<
        OrganizationRepositoryImpl,
        RepositoryRepositoryImpl,
        QuestionRepositoryImpl,
    >
{
    pub fn new(
        org_repo: OrganizationRepositoryImpl,
        repo_repo: RepositoryRepositoryImpl,
        question_repo: QuestionRepositoryImpl,
    ) -> Self {
        Self {
            org_repo,
            repo_repo,
            question_repo,
        }
    }
}

#[async_trait]
impl<O, R, Q> AuthorizationService for AuthorizationServiceImpl<O, R, Q>
where
    O: OrganizationRepository,
    R: RepositoryRepository,
    Q: QuestionRepository,
{
    async fn verify_authorized_for_repository(
        &self,
        request: RepositoryAuthorizationRequest,
    ) -> Result<(), AuthorizationError> {
        let repository = self
            .repo_repo
            .get(request.owner.as_ref(), request.repo.as_ref())
            .await?
            .ok_or_else(|| {
                AuthorizationError::InvalidRequest(format!(
                    "Repository not found: {}",
                    request.get_repo_path(),
                ))
            })?;

        if repository.is_public() {
            return Ok(());
        }

        if request.user_id.is_none() {
            return Err(AuthorizationError::Unauthorized);
        }

        let user_id = request.user_id.unwrap();
        if repository.is_owned_by_user() {
            if repository.owner_id != user_id {
                return Err(AuthorizationError::Unauthorized);
            }
        } else {
            let is_member = self
                .org_repo
                .is_member(repository.owner_id, user_id)
                .await?;
            if !is_member {
                return Err(AuthorizationError::Unauthorized);
            }
        }

        Ok(())
    }

    async fn verify_authorized_for_organization(
        &self,
        request: OrganizationAuthorizationRequest,
    ) -> Result<(), AuthorizationError> {
        let organization = self
            .org_repo
            .get(request.org_name.as_ref())
            .await?
            .ok_or_else(|| {
                AuthorizationError::InvalidRequest(format!(
                    "Organization not found: {}",
                    request.org_name.as_ref()
                ))
            })?;

        if !self
            .org_repo
            .is_admin(organization.id, request.user_id)
            .await?
        {
            return Err(AuthorizationError::Unauthorized);
        }

        Ok(())
    }

    async fn verify_authorized_for_question(
        &self,
        request: QuestionAuthorizationRequest,
    ) -> Result<(), AuthorizationError> {
        let repository = self
            .repo_repo
            .get(request.owner.as_ref(), request.repo.as_ref())
            .await?
            .ok_or_else(|| {
                AuthorizationError::InvalidRequest(format!(
                    "Repository not found: {}",
                    request.get_repo_path(),
                ))
            })?;

        let question_id = self
            .question_repo
            .get_question_id(repository.id, request.number)
            .await?
            .ok_or_else(|| {
                AuthorizationError::InvalidRequest(format!(
                    "Question not found: {}",
                    request.get_repo_path(),
                ))
            })?;

        let author_id = self
            .question_repo
            .get_question_author_id(question_id)
            .await?
            .ok_or_else(|| {
                AuthorizationError::InvalidRequest(format!(
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
                AuthorizationError::InvalidRequest(format!(
                    "Answer not found: {}",
                    request.answer_id
                ))
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
                AuthorizationError::InvalidRequest(format!(
                    "Comment not found: {}",
                    request.comment_id
                ))
            })?;

        if author_id != request.user_id {
            return Err(AuthorizationError::Unauthorized);
        }

        Ok(())
    }
}
