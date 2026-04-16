use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::review::ReviewResource};

pub struct MergeReviewDiff;

impl Endpoint for MergeReviewDiff {
    const PATH: &'static str = "/repository/{owner}/{repo}/review/{number}/diff/{position}/merge";
    const METHOD: http::Method = http::Method::POST;

    type Request = MergeReviewDiffRequest;
    type Response = MergeReviewDiffResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct MergeReviewDiffRequest {}

pub type MergeReviewDiffResponse = ReviewResource;
