use crate::endpoint::Endpoint;
use crate::resource::question::CommentResource;
use serde::{Deserialize, Serialize};

pub struct CreateAnswerComment;

impl Endpoint for CreateAnswerComment {
    const PATH: &'static str = "/repository/{owner}/{repo}/question/{number}/answer/{answer_id}/comment";
    const METHOD: http::Method = http::Method::POST;

    type Request = CreateAnswerCommentRequest;
    type Response = CreateAnswerCommentResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAnswerCommentRequest {
    pub body: String,
}

pub type CreateAnswerCommentResponse = CommentResource;
