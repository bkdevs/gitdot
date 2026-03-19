use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::review::ReviewResource};

pub struct UpdateDiff;

impl Endpoint for UpdateDiff {
    const PATH: &'static str = "/repository/{owner}/{repo}/review/{number}/diff/{position}";
    const METHOD: http::Method = http::Method::PATCH;

    type Request = UpdateDiffRequest;
    type Response = UpdateDiffResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct UpdateDiffRequest {
    pub title: Option<String>,
    pub description: Option<String>,
}

pub type UpdateDiffResponse = ReviewResource;
