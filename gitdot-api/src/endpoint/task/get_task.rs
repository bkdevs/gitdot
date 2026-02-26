use crate::{endpoint::Endpoint, resource::TaskResource};

pub struct GetTask;

impl Endpoint for GetTask {
    const PATH: &'static str = "/ci/task/{id}";
    const METHOD: http::Method = http::Method::GET;

    type Request = ();
    type Response = GetTaskResponse;
}

pub type GetTaskResponse = TaskResource;
