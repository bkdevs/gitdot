use serde::{Deserialize, Serialize};

use super::default_ref;
use crate::{endpoint::Endpoint, resource::repository::RepositoryBlobResource};

pub struct GetRepositoryBlob;

impl Endpoint for GetRepositoryBlob {
    const PATH: &'static str = "/repository/{owner}/{repo}/blob";
    const METHOD: http::Method = http::Method::GET;

    type Request = GetRepositoryBlobRequest;
    type Response = GetRepositoryBlobResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct GetRepositoryBlobRequest {
    #[serde(default = "default_ref")]
    pub ref_name: String,
    pub path: String,
}

pub type GetRepositoryBlobResponse = RepositoryBlobResource;
