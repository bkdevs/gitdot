use api_derive::ApiRequest;
use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::RunnerResource};

pub struct GetRunner;

impl Endpoint for GetRunner {
    const PATH: &'static str = "/ci/runner/{name}";
    const METHOD: http::Method = http::Method::GET;

    type Request = GetRunnerRequest;
    type Response = GetRunnerResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct GetRunnerRequest {
    pub owner_name: String,
    pub owner_type: String,
}

pub type GetRunnerResponse = RunnerResource;
