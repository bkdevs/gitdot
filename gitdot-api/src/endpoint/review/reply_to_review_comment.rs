use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::review::ReviewCommentResource};

pub struct ReplyToReviewComment;

impl Endpoint for ReplyToReviewComment {
    const PATH: &'static str =
        "/repository/{owner}/{repo}/review/{number}/comment/{comment_id}/reply";
    const METHOD: http::Method = http::Method::POST;

    type Request = ReplyToReviewCommentRequest;
    type Response = ReplyToReviewCommentResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct ReplyToReviewCommentRequest {
    pub body: String,
}

pub type ReplyToReviewCommentResponse = ReviewCommentResource;
