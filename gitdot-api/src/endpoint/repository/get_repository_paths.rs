use serde::{Deserialize, Serialize};

use super::default_ref;
use crate::{endpoint::Endpoint, resource::repository::RepositoryPathsResource};

pub struct GetRepositoryPaths;

impl Endpoint for GetRepositoryPaths {
    const PATH: &'static str = "/repository/{owner}/{repo}/paths";
    const METHOD: http::Method = http::Method::GET;

    type Request = GetRepositoryPathsRequest;
    type Response = GetRepositoryPathsResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct GetRepositoryPathsRequest {
    #[serde(default = "default_ref")]
    pub ref_name: String,
}

pub type GetRepositoryPathsResponse = RepositoryPathsResource;
