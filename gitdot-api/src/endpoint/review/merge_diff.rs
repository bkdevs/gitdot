use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::review::ReviewResource};

pub struct MergeDiff;

impl Endpoint for MergeDiff {
    const PATH: &'static str = "/repository/{owner}/{repo}/review/{number}/diff/{position}/merge";
    const METHOD: http::Method = http::Method::POST;

    type Request = MergeDiffRequest;
    type Response = MergeDiffResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct MergeDiffRequest {}

pub type MergeDiffResponse = ReviewResource;
