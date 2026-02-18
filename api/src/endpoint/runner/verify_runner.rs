use crate::endpoint::Endpoint;

pub struct VerifyRunner;

impl Endpoint for VerifyRunner {
    const PATH: &'static str = "/ci/runner/{id}/verify";
    const METHOD: http::Method = http::Method::POST;

    type Request = VerifyRunnerRequest;
    type Response = VerifyRunnerResponse;
}

pub type VerifyRunnerRequest = ();
pub type VerifyRunnerResponse = ();
