use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::endpoint::Endpoint;
use crate::resource::TaskResource;

pub struct PollTask;

impl Endpoint for PollTask {
    const PATH: &'static str = "/ci/task/poll";
    const METHOD: http::Method = http::Method::GET;

    type Request = PollTaskRequest;
    type Response = PollTaskResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PollTaskRequest {
    pub rid: Uuid,
}

pub type PollTaskResponse = TaskResource;
