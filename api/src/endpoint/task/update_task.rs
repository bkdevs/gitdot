use serde::{Deserialize, Serialize};

use crate::endpoint::Endpoint;
use crate::resource::TaskResource;

pub struct UpdateTask;

impl Endpoint for UpdateTask {
    const PATH: &'static str = "/ci/task/{id}";
    const METHOD: http::Method = http::Method::PATCH;

    type Request = UpdateTaskRequest;
    type Response = UpdateTaskResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTaskRequest {
    pub status: String,
}

pub type UpdateTaskResponse = TaskResource;
