use serde::{Deserialize, Serialize};
use api_derive::EndpointRequest;

use crate::{endpoint::Endpoint, resource::question::CommentResource};

pub struct CreateQuestionComment;

impl Endpoint for CreateQuestionComment {
    const PATH: &'static str = "/repository/{owner}/{repo}/question/{number}/comment";
    const METHOD: http::Method = http::Method::POST;

    type Request = CreateQuestionCommentRequest;
    type Response = CreateQuestionCommentResponse;
}

#[derive(EndpointRequest, Debug, Serialize, Deserialize)]
pub struct CreateQuestionCommentRequest {
    pub body: String,
}

pub type CreateQuestionCommentResponse = CommentResource;
