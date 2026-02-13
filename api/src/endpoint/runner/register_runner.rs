use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::endpoint::Endpoint;

pub struct RegisterRunner;

impl Endpoint for RegisterRunner {
    const PATH: &'static str = "/ci/runner/{id}/register";
    const METHOD: http::Method = http::Method::POST;

    type Request = RegisterRunnerRequest;
    type Response = RegisterRunnerResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRunnerRequest {
    pub id: Uuid,
}

pub type RegisterRunnerResponse = ();
