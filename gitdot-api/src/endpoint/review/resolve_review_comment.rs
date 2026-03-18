use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::review::ReviewCommentResource};

pub struct ResolveReviewComment;

impl Endpoint for ResolveReviewComment {
    const PATH: &'static str =
        "/repository/{owner}/{repo}/review/{number}/comment/{comment_id}/resolve";
    const METHOD: http::Method = http::Method::POST;

    type Request = ResolveReviewCommentRequest;
    type Response = ResolveReviewCommentResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct ResolveReviewCommentRequest {
    pub resolved: bool,
}

pub type ResolveReviewCommentResponse = ReviewCommentResource;
