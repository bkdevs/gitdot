use crate::{endpoint::Endpoint, resource::TaskResource};

pub struct ListBuildTasks;

impl Endpoint for ListBuildTasks {
    const PATH: &'static str = "/repository/{owner}/{repo}/build/{number}/tasks";
    const METHOD: http::Method = http::Method::GET;

    type Request = ();
    type Response = ListBuildTasksResponse;
}

pub type ListBuildTasksResponse = Vec<TaskResource>;
