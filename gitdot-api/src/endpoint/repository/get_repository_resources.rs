use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::repository::RepositoryResourcesResource};

pub struct GetRepositoryResources;

impl Endpoint for GetRepositoryResources {
    const PATH: &'static str = "/repository/{owner}/{repo}/resources";
    const METHOD: http::Method = http::Method::POST;

    type Request = GetRepositoryResourcesRequest;
    type Response = GetRepositoryResourcesResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct GetRepositoryResourcesRequest {}

pub type GetRepositoryResourcesResponse = RepositoryResourcesResource;
