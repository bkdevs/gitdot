use serde::{Deserialize, Serialize};
use api_derive::EndpointRequest;

use crate::{endpoint::Endpoint, resource::TaskResource};

pub struct UpdateTask;

impl Endpoint for UpdateTask {
    const PATH: &'static str = "/ci/task/{id}";
    const METHOD: http::Method = http::Method::PATCH;

    type Request = UpdateTaskRequest;
    type Response = UpdateTaskResponse;
}

#[derive(EndpointRequest, Debug, Serialize, Deserialize)]
pub struct UpdateTaskRequest {
    pub status: String,
}

pub type UpdateTaskResponse = TaskResource;
