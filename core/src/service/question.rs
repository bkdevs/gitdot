use async_trait::async_trait;

use crate::dto::{
    AnswerResponse, CommentResponse, CreateAnswerCommentRequest, CreateAnswerRequest,
    CreateQuestionCommentRequest, CreateQuestionRequest, GetQuestionRequest, GetQuestionsRequest,
    QuestionResponse, QuestionsResponse, UpdateAnswerRequest, UpdateCommentRequest,
    UpdateQuestionRequest,
};
use crate::error::QuestionError;
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
    ) -> Result<QuestionsResponse, QuestionError>;

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
            .get_question(repository.id, request.number)
            .await?
            .ok_or_else(|| QuestionError::QuestionNotFound(request.get_question_path()))?;

        Ok(question.into())
    }

    async fn get_questions(
        &self,
        request: GetQuestionsRequest,
    ) -> Result<QuestionsResponse, QuestionError> {
        let repository = self
            .repo_repo
            .get(request.owner.as_ref(), request.repo.as_ref())
            .await?
            .ok_or_else(|| QuestionError::RepositoryNotFound(request.get_repo_path()))?;

        let questions = self.question_repo.get_questions(repository.id).await?;

        Ok(QuestionsResponse {
            questions: questions.into_iter().map(QuestionResponse::from).collect(),
        })
    }

    async fn create_answer(
        &self,
        request: CreateAnswerRequest,
    ) -> Result<AnswerResponse, QuestionError> {
        let repository = self
            .repo_repo
            .get(request.owner.as_ref(), request.repo.as_ref())
            .await?
            .ok_or_else(|| QuestionError::RepositoryNotFound(request.get_repo_path()))?;

        let question = self
            .question_repo
            .get_question(repository.id, request.number)
            .await?
            .ok_or_else(|| QuestionError::QuestionNotFound(request.get_question_path()))?;

        let answer = self
            .question_repo
            .create_answer(question.id, request.author_id, &request.body)
            .await?;

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
        let repository = self
            .repo_repo
            .get(request.owner.as_ref(), request.repo.as_ref())
            .await?
            .ok_or_else(|| QuestionError::RepositoryNotFound(request.get_repo_path()))?;

        let question = self
            .question_repo
            .get_question(repository.id, request.number)
            .await?
            .ok_or_else(|| QuestionError::QuestionNotFound(request.get_question_path()))?;

        let comment = self
            .question_repo
            .create_comment(question.id, request.author_id, &request.body)
            .await?;

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
}
