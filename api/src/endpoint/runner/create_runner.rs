use crate::endpoint::Endpoint;
use crate::resource::CreateRunnerResource;
use serde::{Deserialize, Serialize};

pub struct CreateRunner;

impl Endpoint for CreateRunner {
    const PATH: &'static str = "/ci/runner";
    const METHOD: http::Method = http::Method::POST;

    type Request = CreateRunnerRequest;
    type Response = CreateRunnerResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRunnerRequest {
    pub name: String,
    pub owner_name: String,
    pub owner_type: String,
}

pub type CreateRunnerResponse = CreateRunnerResource;
