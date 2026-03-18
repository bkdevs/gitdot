use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::review::ReviewCommentResource};

pub struct UpdateReviewComment;

impl Endpoint for UpdateReviewComment {
    const PATH: &'static str = "/repository/{owner}/{repo}/review/{number}/comment/{comment_id}";
    const METHOD: http::Method = http::Method::PATCH;

    type Request = UpdateReviewCommentRequest;
    type Response = UpdateReviewCommentResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct UpdateReviewCommentRequest {
    pub body: String,
}

pub type UpdateReviewCommentResponse = ReviewCommentResource;
