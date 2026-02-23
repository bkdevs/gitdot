use api_derive::ApiRequest;
use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::TaskResource};

pub struct UpdateTask;

impl Endpoint for UpdateTask {
    const PATH: &'static str = "/ci/task/{id}";
    const METHOD: http::Method = http::Method::PATCH;

    type Request = UpdateTaskRequest;
    type Response = UpdateTaskResponse;
}

#[derive(ApiRequest, Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTaskRequest {
    pub status: String,
}

pub type UpdateTaskResponse = TaskResource;
