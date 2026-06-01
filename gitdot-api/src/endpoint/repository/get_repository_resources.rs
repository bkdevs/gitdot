use chrono::{DateTime, Utc};
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
pub struct GetRepositoryResourcesRequest {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub last_commit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub last_updated: Option<DateTime<Utc>>,
}

pub type GetRepositoryResourcesResponse = RepositoryResourcesResource;
