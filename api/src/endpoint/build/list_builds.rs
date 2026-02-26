use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::BuildResource};

pub struct ListBuilds;

impl Endpoint for ListBuilds {
    const PATH: &'static str = "/repository/{owner}/{repo}/builds";
    const METHOD: http::Method = http::Method::GET;

    type Request = ListBuildsRequest;
    type Response = ListBuildsResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct ListBuildsRequest {}

pub type ListBuildsResponse = Vec<BuildResource>;
