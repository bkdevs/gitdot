use gitdot_api::resource::question as api;
use gitdot_core::dto::{
    AnswerResponse, AuthorResponse, CommentResponse, QuestionResponse, VoteResponse,
};

use super::IntoApi;

impl IntoApi for QuestionResponse {
    type ApiType = api::QuestionResource;
    fn into_api(self) -> Self::ApiType {
        api::QuestionResource {
            id: self.id,
            number: self.number,
            author_id: self.author_id,
            repository_id: self.repository_id,
            title: self.title,
            body: self.body,
            upvote: self.upvote,
            impression: self.impression,
            created_at: self.created_at,
            updated_at: self.updated_at,
            user_vote: self.user_vote,
            author: self.author.into_api(),
            comments: self.comments.into_api(),
            answers: self.answers.into_api(),
        }
    }
}

impl IntoApi for AnswerResponse {
    type ApiType = api::AnswerResource;
    fn into_api(self) -> Self::ApiType {
        api::AnswerResource {
            id: self.id,
            question_id: self.question_id,
            author_id: self.author_id,
            body: self.body,
            upvote: self.upvote,
            created_at: self.created_at,
            updated_at: self.updated_at,
            user_vote: self.user_vote,
            author: self.author.into_api(),
            comments: self.comments.into_api(),
        }
    }
}

impl IntoApi for CommentResponse {
    type ApiType = api::CommentResource;
    fn into_api(self) -> Self::ApiType {
        api::CommentResource {
            id: self.id,
            parent_id: self.parent_id,
            author_id: self.author_id,
            body: self.body,
            upvote: self.upvote,
            created_at: self.created_at,
            updated_at: self.updated_at,
            user_vote: self.user_vote,
            author: self.author.into_api(),
        }
    }
}

impl IntoApi for AuthorResponse {
    type ApiType = api::AuthorResource;
    fn into_api(self) -> Self::ApiType {
        api::AuthorResource {
            id: self.id,
            name: self.name,
        }
    }
}

impl IntoApi for VoteResponse {
    type ApiType = api::VoteResource;
    fn into_api(self) -> Self::ApiType {
        api::VoteResource {
            target_id: self.target_id,
            score: self.score,
            user_vote: self.user_vote,
        }
    }
}
