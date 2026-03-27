use serde::{Deserialize, Serialize};

use super::default_ref;
use crate::{endpoint::Endpoint, resource::repository::RepositoryBlobsResource};

fn default_refs() -> Vec<String> {
    vec![default_ref()]
}

pub struct GetRepositoryBlobs;

impl Endpoint for GetRepositoryBlobs {
    const PATH: &'static str = "/repository/{owner}/{repo}/blobs";
    const METHOD: http::Method = http::Method::POST;

    type Request = GetRepositoryBlobsRequest;
    type Response = GetRepositoryBlobsResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct GetRepositoryBlobsRequest {
    #[serde(default = "default_refs")]
    pub refs: Vec<String>,
    pub paths: Vec<String>,
}

pub type GetRepositoryBlobsResponse = RepositoryBlobsResource;
