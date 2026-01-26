use async_trait::async_trait;

use crate::dto::{
    AnswerResponse, CommentResponse, CreateAnswerRequest, CreateCommentRequest,
    CreateQuestionRequest, GetQuestionRequest, GetQuestionsRequest, QuestionResponse,
    QuestionsResponse, UpdateAnswerRequest, UpdateCommentRequest, UpdateQuestionRequest,
};
use crate::error::QuestionError;
use crate::repository::{QuestionRepository, QuestionRepositoryImpl};

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

    async fn create_comment(
        &self,
        request: CreateCommentRequest,
    ) -> Result<CommentResponse, QuestionError>;

    async fn update_comment(
        &self,
        request: UpdateCommentRequest,
    ) -> Result<CommentResponse, QuestionError>;
}

#[derive(Debug, Clone)]
pub struct QuestionServiceImpl<Q>
where
    Q: QuestionRepository,
{
    question_repo: Q,
}

impl QuestionServiceImpl<QuestionRepositoryImpl> {
    pub fn new(question_repo: QuestionRepositoryImpl) -> Self {
        Self { question_repo }
    }
}

#[async_trait]
impl<Q> QuestionService for QuestionServiceImpl<Q>
where
    Q: QuestionRepository,
{
    async fn create_question(
        &self,
        request: CreateQuestionRequest,
    ) -> Result<QuestionResponse, QuestionError> {
        let question = self
            .question_repo
            .create_question(
                request.author_id,
                request.repository_id,
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
        let question = self
            .question_repo
            .update_question(request.id, &request.title, &request.body)
            .await?
            .ok_or_else(|| QuestionError::QuestionNotFound(request.id))?;

        Ok(question.into())
    }

    async fn get_question(
        &self,
        request: GetQuestionRequest,
    ) -> Result<QuestionResponse, QuestionError> {
        let question = self
            .question_repo
            .get_question(request.id)
            .await?
            .ok_or_else(|| QuestionError::QuestionNotFound(request.id))?;

        Ok(question.into())
    }

    async fn get_questions(
        &self,
        request: GetQuestionsRequest,
    ) -> Result<QuestionsResponse, QuestionError> {
        let questions = self
            .question_repo
            .get_questions(request.repository_id)
            .await?;

        Ok(QuestionsResponse {
            questions: questions.into_iter().map(QuestionResponse::from).collect(),
        })
    }

    async fn create_answer(
        &self,
        request: CreateAnswerRequest,
    ) -> Result<AnswerResponse, QuestionError> {
        let answer = self
            .question_repo
            .create_answer(request.question_id, request.author_id, &request.body)
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

    async fn create_comment(
        &self,
        request: CreateCommentRequest,
    ) -> Result<CommentResponse, QuestionError> {
        let comment = self
            .question_repo
            .create_comment(request.parent_id, request.author_id, &request.body)
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
