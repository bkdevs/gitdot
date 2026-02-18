use crate::{endpoint::Endpoint, resource::runner::RunnerTokenResource};

pub struct CreateRunnerToken;

impl Endpoint for CreateRunnerToken {
    const PATH: &'static str = "/ci/runner/{id}/token";
    const METHOD: http::Method = http::Method::POST;

    type Request = CreateRunnerTokenRequest;
    type Response = CreateRunnerTokenResponse;
}

pub type CreateRunnerTokenRequest = ();
pub type CreateRunnerTokenResponse = RunnerTokenResource;
