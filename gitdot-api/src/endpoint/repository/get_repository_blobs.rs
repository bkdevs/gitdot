use serde::{Deserialize, Serialize};

use super::default_ref;
use crate::{endpoint::Endpoint, resource::repository::RepositoryBlobsResource};

pub struct GetRepositoryBlobs;

impl Endpoint for GetRepositoryBlobs {
    const PATH: &'static str = "/repository/{owner}/{repo}/blobs";
    const METHOD: http::Method = http::Method::POST;

    type Request = GetRepositoryBlobsRequest;
    type Response = GetRepositoryBlobsResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct GetRepositoryBlobsRequest {
    #[serde(default = "default_ref")]
    pub ref_name: String,
    pub paths: Vec<String>,
}

pub type GetRepositoryBlobsResponse = RepositoryBlobsResource;
