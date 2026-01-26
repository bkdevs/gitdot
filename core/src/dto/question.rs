mod create_answer;
mod create_answer_comment;
mod create_question;
mod create_question_comment;
mod get_question;
mod get_questions;
mod update_answer;
mod update_comment;
mod update_question;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::model::{Answer, Comment, Question, User};

pub use create_answer::CreateAnswerRequest;
pub use create_answer_comment::CreateAnswerCommentRequest;
pub use create_question::CreateQuestionRequest;
pub use create_question_comment::CreateQuestionCommentRequest;
pub use get_question::GetQuestionRequest;
pub use get_questions::GetQuestionsRequest;
pub use update_answer::UpdateAnswerRequest;
pub use update_comment::UpdateCommentRequest;
pub use update_question::UpdateQuestionRequest;

#[derive(Debug, Clone)]
pub struct QuestionsResponse {
    pub questions: Vec<QuestionResponse>,
}

#[derive(Debug, Clone)]
pub struct QuestionResponse {
    pub id: Uuid,
    pub number: i32,
    pub author_id: Uuid,
    pub repository_id: Uuid,
    pub title: String,
    pub body: String,
    pub upvote: i32,
    pub impression: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub author: Option<AuthorResponse>,
    pub comments: Vec<CommentResponse>,
    pub answers: Vec<AnswerResponse>,
}

impl From<Question> for QuestionResponse {
    fn from(question: Question) -> Self {
        Self {
            id: question.id,
            number: question.number,
            author_id: question.author_id,
            repository_id: question.repository_id,
            title: question.title,
            body: question.body,
            upvote: question.upvote,
            impression: question.impression,
            created_at: question.created_at,
            updated_at: question.updated_at,
            author: question.author.map(AuthorResponse::from),
            comments: question
                .comments
                .unwrap_or_default()
                .into_iter()
                .map(CommentResponse::from)
                .collect(),
            answers: question
                .answers
                .unwrap_or_default()
                .into_iter()
                .map(AnswerResponse::from)
                .collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AnswerResponse {
    pub id: Uuid,
    pub question_id: Uuid,
    pub author_id: Uuid,
    pub body: String,
    pub upvote: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub author: Option<AuthorResponse>,
    pub comments: Vec<CommentResponse>,
}

impl From<Answer> for AnswerResponse {
    fn from(answer: Answer) -> Self {
        Self {
            id: answer.id,
            question_id: answer.question_id,
            author_id: answer.author_id,
            body: answer.body,
            upvote: answer.upvote,
            created_at: answer.created_at,
            updated_at: answer.updated_at,
            author: answer.author.map(AuthorResponse::from),
            comments: answer
                .comments
                .unwrap_or_default()
                .into_iter()
                .map(CommentResponse::from)
                .collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CommentResponse {
    pub id: Uuid,
    pub parent_id: Uuid,
    pub author_id: Uuid,
    pub body: String,
    pub upvote: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub author: Option<AuthorResponse>,
}

impl From<Comment> for CommentResponse {
    fn from(comment: Comment) -> Self {
        Self {
            id: comment.id,
            parent_id: comment.parent_id,
            author_id: comment.author_id,
            body: comment.body,
            upvote: comment.upvote,
            created_at: comment.created_at,
            updated_at: comment.updated_at,
            author: comment.author.map(AuthorResponse::from),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AuthorResponse {
    pub id: Uuid,
    pub name: String,
}

impl From<User> for AuthorResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
        }
    }
}
