use crate::{endpoint::Endpoint, resource::RunnerResource};

pub struct GetRunner;

impl Endpoint for GetRunner {
    const PATH: &'static str = "/ci/runner/{owner}/{name}";
    const METHOD: http::Method = http::Method::GET;

    type Request = GetRunnerRequest;
    type Response = GetRunnerResponse;
}

pub type GetRunnerRequest = ();
pub type GetRunnerResponse = RunnerResource;
