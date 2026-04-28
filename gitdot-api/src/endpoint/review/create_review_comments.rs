use serde::{Deserialize, Serialize};

use crate::{
    endpoint::{review_review_diff::ReviewCommentInput, Endpoint},
    resource::review::ReviewResource,
};

pub struct CreateReviewComments;

impl Endpoint for CreateReviewComments {
    const PATH: &'static str =
        "/repository/{owner}/{repo}/review/{number}/diff/{position}/comments";
    const METHOD: http::Method = http::Method::POST;

    type Request = CreateReviewCommentsRequest;
    type Response = CreateReviewCommentsResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct CreateReviewCommentsRequest {
    pub comments: Vec<ReviewCommentInput>,
}

pub type CreateReviewCommentsResponse = ReviewResource;
