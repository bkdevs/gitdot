use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::review::ReviewResource};

pub struct UpdateReviewDiff;

impl Endpoint for UpdateReviewDiff {
    const PATH: &'static str = "/repository/{owner}/{repo}/review/{number}/diff/{position}";
    const METHOD: http::Method = http::Method::PATCH;

    type Request = UpdateReviewDiffRequest;
    type Response = UpdateReviewDiffResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct UpdateReviewDiffRequest {
    pub message: Option<String>,
}

pub type UpdateReviewDiffResponse = ReviewResource;
