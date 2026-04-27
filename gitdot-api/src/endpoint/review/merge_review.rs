use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::review::ReviewResource};

pub struct MergeReview;

impl Endpoint for MergeReview {
    const PATH: &'static str = "/repository/{owner}/{repo}/review/{number}/merge";
    const METHOD: http::Method = http::Method::POST;

    type Request = MergeReviewRequest;
    type Response = MergeReviewResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct MergeReviewRequest {}

pub type MergeReviewResponse = ReviewResource;
