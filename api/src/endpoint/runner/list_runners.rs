use crate::{endpoint::Endpoint, resource::RunnerResource};

pub struct ListRunners;

impl Endpoint for ListRunners {
    const PATH: &'static str = "/ci/runner/{owner}";
    const METHOD: http::Method = http::Method::GET;

    type Request = ();
    type Response = Vec<RunnerResource>;
}

pub type ListRunnersRequest = ();
pub type ListRunnersResponse = Vec<RunnerResource>;
