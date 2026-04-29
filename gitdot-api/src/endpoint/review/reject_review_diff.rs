use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::review::ReviewResource};

pub struct RejectReviewDiff;

impl Endpoint for RejectReviewDiff {
    const PATH: &'static str = "/repository/{owner}/{repo}/review/{number}/diff/{position}/reject";
    const METHOD: http::Method = http::Method::POST;

    type Request = RejectReviewDiffRequest;
    type Response = RejectReviewDiffResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct RejectReviewDiffRequest {}

pub type RejectReviewDiffResponse = ReviewResource;
