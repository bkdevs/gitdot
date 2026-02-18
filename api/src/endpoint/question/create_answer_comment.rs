use api_derive::ApiRequest;
use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::question::CommentResource};

pub struct CreateAnswerComment;

impl Endpoint for CreateAnswerComment {
    const PATH: &'static str =
        "/repository/{owner}/{repo}/question/{number}/answer/{answer_id}/comment";
    const METHOD: http::Method = http::Method::POST;

    type Request = CreateAnswerCommentRequest;
    type Response = CreateAnswerCommentResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct CreateAnswerCommentRequest {
    pub body: String,
}

pub type CreateAnswerCommentResponse = CommentResource;
