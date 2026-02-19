use crate::endpoint::Endpoint;

pub struct DeleteRunner;

impl Endpoint for DeleteRunner {
    const PATH: &'static str = "/ci/runner/{owner}/{name}";
    const METHOD: http::Method = http::Method::DELETE;

    type Request = DeleteRunnerRequest;
    type Response = DeleteRunnerResponse;
}

pub type DeleteRunnerRequest = ();
pub type DeleteRunnerResponse = ();
