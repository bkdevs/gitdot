use serde::{Deserialize, Serialize};

use api_derive::ApiRequest;

use crate::{endpoint::Endpoint, resource::BuildResource};

pub struct ListBuilds;

impl Endpoint for ListBuilds {
    const PATH: &'static str = "/ci/builds";
    const METHOD: http::Method = http::Method::GET;

    type Request = ListBuildsRequest;
    type Response = ListBuildsResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct ListBuildsRequest {
    pub owner: String,
    pub repo: String,
}

pub type ListBuildsResponse = Vec<BuildResource>;
