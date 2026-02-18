use serde::{Deserialize, Serialize};
use api_derive::EndpointRequest;

use crate::{endpoint::Endpoint, resource::DagResource};

pub struct CreateDag;

impl Endpoint for CreateDag {
    const PATH: &'static str = "/ci/dag";
    const METHOD: http::Method = http::Method::POST;

    type Request = CreateDagRequest;
    type Response = CreateDagResponse;
}

#[derive(EndpointRequest, Debug, Serialize, Deserialize)]
pub struct CreateDagRequest {
    pub repo_owner: String,
    pub repo_name: String,
}

pub type CreateDagResponse = DagResource;
