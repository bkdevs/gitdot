use serde::{Deserialize, Serialize};
use api_derive::EndpointRequest;

use crate::{endpoint::Endpoint, resource::RunnerResource};

pub struct CreateRunner;

impl Endpoint for CreateRunner {
    const PATH: &'static str = "/ci/runner";
    const METHOD: http::Method = http::Method::POST;

    type Request = CreateRunnerRequest;
    type Response = CreateRunnerResponse;
}

#[derive(EndpointRequest, Debug, Serialize, Deserialize)]
pub struct CreateRunnerRequest {
    pub name: String,
    pub owner_name: String,
    pub owner_type: String,
}

pub type CreateRunnerResponse = RunnerResource;
