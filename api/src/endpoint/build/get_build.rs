use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::GetBuildResource};

pub struct GetBuild;

impl Endpoint for GetBuild {
    const PATH: &'static str = "/repository/{owner}/{repo}/build/{number}";
    const METHOD: http::Method = http::Method::GET;

    type Request = GetBuildRequest;
    type Response = GetBuildResource;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct GetBuildRequest;

pub type GetBuildResponse = GetBuildResource;
