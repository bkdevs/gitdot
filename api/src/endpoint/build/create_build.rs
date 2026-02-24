use std::collections::HashMap;

use api_derive::ApiRequest;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{endpoint::Endpoint, resource::BuildResource};

pub struct CreateBuild;

impl Endpoint for CreateBuild {
    const PATH: &'static str = "/ci/build";
    const METHOD: http::Method = http::Method::POST;

    type Request = CreateBuildRequest;
    type Response = CreateBuildResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct CreateBuildRequest {
    pub repo_owner: String,
    pub repo_name: String,
    pub task_dependencies: HashMap<Uuid, Vec<Uuid>>,
}

pub type CreateBuildResponse = BuildResource;
