use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::repository::RepositoryDiffResource};

pub struct GetReviewDiff;

impl Endpoint for GetReviewDiff {
    const PATH: &'static str = "/repository/{owner}/{repo}/review/{number}/diff/{position}";
    const METHOD: http::Method = http::Method::GET;

    type Request = GetReviewDiffRequest;
    type Response = GetReviewDiffResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct GetReviewDiffRequest {}

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GetReviewDiffResponse {
    pub files: Vec<RepositoryDiffResource>,
}
