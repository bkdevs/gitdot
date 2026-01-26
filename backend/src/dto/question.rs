mod create_answer;
mod create_comment;
mod create_question;
mod update_answer;
mod update_comment;
mod update_question;
mod vote;

use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use gitdot_core::dto::{
    AnswerResponse, AuthorResponse, CommentResponse, QuestionResponse, QuestionsResponse,
};

pub use create_answer::CreateAnswerServerRequest;
pub use create_comment::CreateCommentServerRequest;
pub use create_question::CreateQuestionServerRequest;
pub use update_answer::UpdateAnswerServerRequest;
pub use update_comment::UpdateCommentServerRequest;
pub use update_question::UpdateQuestionServerRequest;
pub use vote::{VoteServerRequest, VoteServerResponse};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct QuestionsServerResponse {
    pub questions: Vec<QuestionServerResponse>,
}

impl From<QuestionsResponse> for QuestionsServerResponse {
    fn from(response: QuestionsResponse) -> Self {
        Self {
            questions: response.questions.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct QuestionServerResponse {
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
    pub user_vote: Option<i16>,
    pub author: Option<AuthorServerResponse>,
    pub comments: Vec<CommentServerResponse>,
    pub answers: Vec<AnswerServerResponse>,
}

impl From<QuestionResponse> for QuestionServerResponse {
    fn from(response: QuestionResponse) -> Self {
        Self {
            id: response.id,
            number: response.number,
            author_id: response.author_id,
            repository_id: response.repository_id,
            title: response.title,
            body: response.body,
            upvote: response.upvote,
            impression: response.impression,
            created_at: response.created_at,
            updated_at: response.updated_at,
            user_vote: response.user_vote,
            author: response.author.map(AuthorServerResponse::from),
            comments: response
                .comments
                .into_iter()
                .map(CommentServerResponse::from)
                .collect(),
            answers: response
                .answers
                .into_iter()
                .map(AnswerServerResponse::from)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AnswerServerResponse {
    pub id: Uuid,
    pub question_id: Uuid,
    pub author_id: Uuid,
    pub body: String,
    pub upvote: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user_vote: Option<i16>,
    pub author: Option<AuthorServerResponse>,
    pub comments: Vec<CommentServerResponse>,
}

impl From<AnswerResponse> for AnswerServerResponse {
    fn from(response: AnswerResponse) -> Self {
        Self {
            id: response.id,
            question_id: response.question_id,
            author_id: response.author_id,
            body: response.body,
            upvote: response.upvote,
            created_at: response.created_at,
            updated_at: response.updated_at,
            user_vote: response.user_vote,
            author: response.author.map(AuthorServerResponse::from),
            comments: response
                .comments
                .into_iter()
                .map(CommentServerResponse::from)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommentServerResponse {
    pub id: Uuid,
    pub parent_id: Uuid,
    pub author_id: Uuid,
    pub body: String,
    pub upvote: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user_vote: Option<i16>,
    pub author: Option<AuthorServerResponse>,
}

impl From<CommentResponse> for CommentServerResponse {
    fn from(response: CommentResponse) -> Self {
        Self {
            id: response.id,
            parent_id: response.parent_id,
            author_id: response.author_id,
            body: response.body,
            upvote: response.upvote,
            created_at: response.created_at,
            updated_at: response.updated_at,
            user_vote: response.user_vote,
            author: response.author.map(AuthorServerResponse::from),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AuthorServerResponse {
    pub id: Uuid,
    pub name: String,
}

impl From<AuthorResponse> for AuthorServerResponse {
    fn from(response: AuthorResponse) -> Self {
        Self {
            id: response.id,
            name: response.name,
        }
    }
}
