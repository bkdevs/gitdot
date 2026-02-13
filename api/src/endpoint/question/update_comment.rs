use crate::endpoint::Endpoint;
use crate::resource::question::CommentResource;
use serde::{Deserialize, Serialize};

pub struct UpdateComment;

impl Endpoint for UpdateComment {
    const PATH: &'static str = "/repository/{owner}/{repo}/question/{number}/comment/{comment_id}";
    const METHOD: http::Method = http::Method::PATCH;

    type Request = UpdateCommentRequest;
    type Response = UpdateCommentResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCommentRequest {
    pub body: String,
}

pub type UpdateCommentResponse = CommentResource;
