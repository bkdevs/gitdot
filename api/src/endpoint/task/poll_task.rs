use crate::{endpoint::Endpoint, resource::TaskResource};

pub struct PollTask;

impl Endpoint for PollTask {
    const PATH: &'static str = "/ci/task/poll";
    const METHOD: http::Method = http::Method::GET;

    type Request = PollTaskRequest;
    type Response = PollTaskResponse;
}

pub type PollTaskRequest = ();

pub type PollTaskResponse = TaskResource;
