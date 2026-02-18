use api_derive::ApiRequest;
use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::question::CommentResource};

pub struct UpdateComment;

impl Endpoint for UpdateComment {
    const PATH: &'static str = "/repository/{owner}/{repo}/question/{number}/comment/{comment_id}";
    const METHOD: http::Method = http::Method::PATCH;

    type Request = UpdateCommentRequest;
    type Response = UpdateCommentResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct UpdateCommentRequest {
    pub body: String,
}

pub type UpdateCommentResponse = CommentResource;
