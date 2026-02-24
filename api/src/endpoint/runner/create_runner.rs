use serde::{Deserialize, Serialize};

use api_derive::ApiRequest;

use crate::{endpoint::Endpoint, resource::RunnerResource};

pub struct CreateRunner;

impl Endpoint for CreateRunner {
    const PATH: &'static str = "/ci/runner/{owner}";
    const METHOD: http::Method = http::Method::POST;

    type Request = CreateRunnerRequest;
    type Response = CreateRunnerResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct CreateRunnerRequest {
    pub name: String,
    pub owner_type: String,
}

pub type CreateRunnerResponse = RunnerResource;
