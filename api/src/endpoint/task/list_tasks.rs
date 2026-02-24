use serde::{Deserialize, Serialize};

use api_derive::ApiRequest;

use crate::{endpoint::Endpoint, resource::TaskResource};

pub struct ListTasks;

impl Endpoint for ListTasks {
    const PATH: &'static str = "/ci/tasks";
    const METHOD: http::Method = http::Method::GET;

    type Request = ListTasksRequest;
    type Response = ListTasksResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct ListTasksRequest {
    pub owner: String,
    pub repo: String,
}

pub type ListTasksResponse = Vec<TaskResource>;
