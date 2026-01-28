use async_trait::async_trait;

use crate::dto::{
    AnswerResponse, CommentResponse, CreateAnswerCommentRequest, CreateAnswerRequest,
    CreateQuestionCommentRequest, CreateQuestionRequest, GetQuestionRequest, GetQuestionsRequest,
    QuestionResponse, UpdateAnswerRequest, UpdateCommentRequest, UpdateQuestionRequest,
    VoteAnswerRequest, VoteCommentRequest, VoteQuestionRequest, VoteResponse,
};
use crate::error::QuestionError;
use crate::model::VoteTarget;
use crate::repository::{
    QuestionRepository, QuestionRepositoryImpl, RepositoryRepository, RepositoryRepositoryImpl,
};

#[async_trait]
pub trait QuestionService: Send + Sync + 'static {
    async fn create_question(
        &self,
        request: CreateQuestionRequest,
    ) -> Result<QuestionResponse, QuestionError>;

    async fn update_question(
        &self,
        request: UpdateQuestionRequest,
    ) -> Result<QuestionResponse, QuestionError>;

    async fn get_question(
        &self,
        request: GetQuestionRequest,
    ) -> Result<QuestionResponse, QuestionError>;

    async fn get_questions(
        &self,
        request: GetQuestionsRequest,
    ) -> Result<Vec<QuestionResponse>, QuestionError>;

    async fn create_answer(
        &self,
        request: CreateAnswerRequest,
    ) -> Result<AnswerResponse, QuestionError>;

    async fn update_answer(
        &self,
        request: UpdateAnswerRequest,
    ) -> Result<AnswerResponse, QuestionError>;

    async fn create_question_comment(
        &self,
        request: CreateQuestionCommentRequest,
    ) -> Result<CommentResponse, QuestionError>;

    async fn create_answer_comment(
        &self,
        request: CreateAnswerCommentRequest,
    ) -> Result<CommentResponse, QuestionError>;

    async fn update_comment(
        &self,
        request: UpdateCommentRequest,
    ) -> Result<CommentResponse, QuestionError>;

    async fn vote_question(
        &self,
        request: VoteQuestionRequest,
    ) -> Result<VoteResponse, QuestionError>;

    async fn vote_answer(&self, request: VoteAnswerRequest) -> Result<VoteResponse, QuestionError>;

    async fn vote_comment(
        &self,
        request: VoteCommentRequest,
    ) -> Result<VoteResponse, QuestionError>;
}

#[derive(Debug, Clone)]
pub struct QuestionServiceImpl<Q, R>
where
    Q: QuestionRepository,
    R: RepositoryRepository,
{
    question_repo: Q,
    repo_repo: R,
}

impl QuestionServiceImpl<QuestionRepositoryImpl, RepositoryRepositoryImpl> {
    pub fn new(question_repo: QuestionRepositoryImpl, repo_repo: RepositoryRepositoryImpl) -> Self {
        Self {
            question_repo,
            repo_repo,
        }
    }
}

#[async_trait]
impl<Q, R> QuestionService for QuestionServiceImpl<Q, R>
where
    Q: QuestionRepository,
    R: RepositoryRepository,
{
    async fn create_question(
        &self,
        request: CreateQuestionRequest,
    ) -> Result<QuestionResponse, QuestionError> {
        let repository = self
            .repo_repo
            .get(request.owner.as_ref(), request.repo.as_ref())
            .await?
            .ok_or_else(|| QuestionError::RepositoryNotFound(request.get_repo_path()))?;

        let question = self
            .question_repo
            .create_question(
                request.author_id,
                repository.id,
                &request.title,
                &request.body,
            )
            .await?;

        Ok(question.into())
    }

    async fn update_question(
        &self,
        request: UpdateQuestionRequest,
    ) -> Result<QuestionResponse, QuestionError> {
        let repository = self
            .repo_repo
            .get(request.owner.as_ref(), request.repo.as_ref())
            .await?
            .ok_or_else(|| QuestionError::RepositoryNotFound(request.get_repo_path()))?;

        let question = self
            .question_repo
            .update_question(repository.id, request.number, &request.title, &request.body)
            .await?
            .ok_or_else(|| QuestionError::QuestionNotFound(request.get_question_path()))?;

        Ok(question.into())
    }

    async fn get_question(
        &self,
        request: GetQuestionRequest,
    ) -> Result<QuestionResponse, QuestionError> {
        let repository = self
            .repo_repo
            .get(request.owner.as_ref(), request.repo.as_ref())
            .await?
            .ok_or_else(|| QuestionError::RepositoryNotFound(request.get_repo_path()))?;

        let question = self
            .question_repo
            .get_question(repository.id, request.number, request.user_id)
            .await?
            .ok_or_else(|| QuestionError::QuestionNotFound(request.get_question_path()))?;

        Ok(question.into())
    }

    async fn get_questions(
        &self,
        request: GetQuestionsRequest,
    ) -> Result<Vec<QuestionResponse>, QuestionError> {
        let repository = self
            .repo_repo
            .get(request.owner.as_ref(), request.repo.as_ref())
            .await?
            .ok_or_else(|| QuestionError::RepositoryNotFound(request.get_repo_path()))?;

        let questions = self
            .question_repo
            .get_questions(repository.id, request.user_id)
            .await?;

        Ok(questions.into_iter().map(QuestionResponse::from).collect())
    }

    async fn create_answer(
        &self,
        request: CreateAnswerRequest,
    ) -> Result<AnswerResponse, QuestionError> {
        let answer = self
            .question_repo
            .create_answer(
                request.owner.as_ref(),
                request.repo.as_ref(),
                request.number,
                request.author_id,
                &request.body,
            )
            .await?
            .ok_or_else(|| QuestionError::QuestionNotFound(request.get_question_path()))?;

        Ok(answer.into())
    }

    async fn update_answer(
        &self,
        request: UpdateAnswerRequest,
    ) -> Result<AnswerResponse, QuestionError> {
        let answer = self
            .question_repo
            .update_answer(request.id, &request.body)
            .await?
            .ok_or_else(|| QuestionError::AnswerNotFound(request.id))?;

        Ok(answer.into())
    }

    async fn create_question_comment(
        &self,
        request: CreateQuestionCommentRequest,
    ) -> Result<CommentResponse, QuestionError> {
        let comment = self
            .question_repo
            .create_question_comment(
                request.owner.as_ref(),
                request.repo.as_ref(),
                request.number,
                request.author_id,
                &request.body,
            )
            .await?
            .ok_or_else(|| QuestionError::QuestionNotFound(request.get_question_path()))?;

        Ok(comment.into())
    }

    async fn create_answer_comment(
        &self,
        request: CreateAnswerCommentRequest,
    ) -> Result<CommentResponse, QuestionError> {
        let comment = self
            .question_repo
            .create_comment(request.answer_id, request.author_id, &request.body)
            .await?;

        Ok(comment.into())
    }

    async fn update_comment(
        &self,
        request: UpdateCommentRequest,
    ) -> Result<CommentResponse, QuestionError> {
        let comment = self
            .question_repo
            .update_comment(request.id, &request.body)
            .await?
            .ok_or_else(|| QuestionError::CommentNotFound(request.id))?;

        Ok(comment.into())
    }

    async fn vote_question(
        &self,
        request: VoteQuestionRequest,
    ) -> Result<VoteResponse, QuestionError> {
        let question_id = self
            .question_repo
            .get_question_id(
                request.owner.as_ref(),
                request.repo.as_ref(),
                request.number,
            )
            .await?
            .ok_or_else(|| QuestionError::QuestionNotFound(request.get_question_path()))?;

        let result = self
            .question_repo
            .vote(
                request.user_id,
                question_id,
                VoteTarget::Question,
                request.value,
            )
            .await?;

        Ok(result.into())
    }

    async fn vote_answer(&self, request: VoteAnswerRequest) -> Result<VoteResponse, QuestionError> {
        let result = self
            .question_repo
            .vote(
                request.user_id,
                request.answer_id,
                VoteTarget::Answer,
                request.value,
            )
            .await?;

        Ok(result.into())
    }

    async fn vote_comment(
        &self,
        request: VoteCommentRequest,
    ) -> Result<VoteResponse, QuestionError> {
        let result = self
            .question_repo
            .vote(
                request.user_id,
                request.comment_id,
                VoteTarget::Comment,
                request.value,
            )
            .await?;

        Ok(result.into())
    }
}
