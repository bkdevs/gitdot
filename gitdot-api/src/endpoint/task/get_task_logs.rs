use crate::{endpoint::Endpoint, resource::task::TaskLogResource};

pub struct GetTaskLogs;

impl Endpoint for GetTaskLogs {
    const PATH: &'static str = "/ci/task/{id}/logs";
    const METHOD: http::Method = http::Method::GET;

    type Request = GetTaskLogsRequest;
    type Response = GetTaskLogsResponse;
}

pub type GetTaskLogsRequest = ();

pub type GetTaskLogsResponse = Vec<TaskLogResource>;
