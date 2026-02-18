use api_derive::ApiRequest;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{endpoint::Endpoint, resource::TaskResource};

pub struct PollTask;

impl Endpoint for PollTask {
    const PATH: &'static str = "/ci/task/poll";
    const METHOD: http::Method = http::Method::GET;

    type Request = PollTaskRequest;
    type Response = PollTaskResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct PollTaskRequest {
    pub rid: Uuid,
}

pub type PollTaskResponse = TaskResource;
