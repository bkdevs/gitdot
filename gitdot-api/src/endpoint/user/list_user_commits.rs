use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::repository::RepositoryCommitResource};

pub struct ListUserCommits;

impl Endpoint for ListUserCommits {
    const PATH: &'static str = "/user/{user_name}/commits";
    const METHOD: http::Method = http::Method::GET;

    type Request = ListUserCommitsRequest;
    type Response = ListUserCommitsResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct ListUserCommitsRequest {}

pub type ListUserCommitsResponse = Vec<RepositoryCommitResource>;
