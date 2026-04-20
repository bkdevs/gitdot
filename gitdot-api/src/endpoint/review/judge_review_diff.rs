use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::review::ReviewResource};

pub struct JudgeReviewDiff;

impl Endpoint for JudgeReviewDiff {
    const PATH: &'static str = "/repository/{owner}/{repo}/review/{number}/diff/{position}/submit";
    const METHOD: http::Method = http::Method::POST;

    type Request = JudgeReviewDiffRequest;
    type Response = JudgeReviewDiffResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct JudgeReviewDiffRequest {
    pub verdict: String,
}

pub type JudgeReviewDiffResponse = ReviewResource;
