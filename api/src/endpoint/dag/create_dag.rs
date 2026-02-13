use crate::endpoint::Endpoint;
use crate::resource::DagResource;
use serde::{Deserialize, Serialize};

pub struct CreateDag;

impl Endpoint for CreateDag {
    const PATH: &'static str = "/ci/dag";
    const METHOD: http::Method = http::Method::POST;

    type Request = CreateDagRequest;
    type Response = CreateDagResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDagRequest {
    pub repo_owner: String,
    pub repo_name: String,
}

pub type CreateDagResponse = DagResource;
