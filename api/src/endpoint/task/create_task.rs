use serde::{Deserialize, Serialize};
use uuid::Uuid;

use api_derive::ApiRequest;

use crate::{endpoint::Endpoint, resource::TaskResource};

pub struct CreateTask;

impl Endpoint for CreateTask {
    const PATH: &'static str = "/ci/task";
    const METHOD: http::Method = http::Method::POST;

    type Request = CreateTaskRequest;
    type Response = CreateTaskResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct CreateTaskRequest {
    pub repo_owner: String,
    pub repo_name: String,
    pub name: String,
    pub script: String,
    pub build_id: Uuid,
}

pub type CreateTaskResponse = TaskResource;
