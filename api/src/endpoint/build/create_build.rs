use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::BuildResource};

pub struct CreateBuild;

impl Endpoint for CreateBuild {
    const PATH: &'static str = "/repository/{owner}/{repo}/build";
    const METHOD: http::Method = http::Method::POST;

    type Request = CreateBuildRequest;
    type Response = CreateBuildResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct CreateBuildRequest {
    pub trigger: String,
    pub commit_sha: String,
}

pub type CreateBuildResponse = BuildResource;
