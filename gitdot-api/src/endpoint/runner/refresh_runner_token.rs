use crate::{endpoint::Endpoint, resource::runner::RunnerTokenResource};

pub struct CreateRunnerToken;

impl Endpoint for CreateRunnerToken {
    const PATH: &'static str = "/ci/runner/{owner}/{name}/token";
    const METHOD: http::Method = http::Method::POST;

    type Request = RefreshRunnerTokenRequest;
    type Response = RefreshRunnerTokenResponse;
}

pub type RefreshRunnerTokenRequest = ();
pub type RefreshRunnerTokenResponse = RunnerTokenResource;
