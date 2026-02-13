use crate::endpoint::Endpoint;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct DeleteRunner;

impl Endpoint for DeleteRunner {
    const PATH: &'static str = "/ci/runner/{id}";
    const METHOD: http::Method = http::Method::DELETE;

    type Request = DeleteRunnerRequest;
    type Response = DeleteRunnerResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteRunnerRequest {
    pub id: Uuid,
}

pub type DeleteRunnerResponse = ();
