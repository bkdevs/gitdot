use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::GetBuildByNumberResource};

pub struct GetBuildByNumber;

impl Endpoint for GetBuildByNumber {
    const PATH: &'static str = "/ci/builds/{owner}/{repo}/{number}";
    const METHOD: http::Method = http::Method::GET;

    type Request = GetBuildByNumberRequest;
    type Response = GetBuildByNumberResource;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct GetBuildByNumberRequest;

pub type GetBuildByNumberResponse = GetBuildByNumberResource;
