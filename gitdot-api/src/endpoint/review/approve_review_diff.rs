use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::review::ReviewResource};

pub struct ApproveReviewDiff;

impl Endpoint for ApproveReviewDiff {
    const PATH: &'static str =
        "/repository/{owner}/{repo}/review/{number}/diff/{position}/approve";
    const METHOD: http::Method = http::Method::POST;

    type Request = ApproveReviewDiffRequest;
    type Response = ApproveReviewDiffResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct ApproveReviewDiffRequest {}

pub type ApproveReviewDiffResponse = ReviewResource;
