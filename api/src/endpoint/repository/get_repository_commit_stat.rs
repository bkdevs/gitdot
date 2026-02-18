use serde::{Deserialize, Serialize};
use api_derive::EndpointRequest;

use crate::{endpoint::Endpoint, resource::repository::RepositoryCommitStatResource};

pub struct GetRepositoryCommitStat;

impl Endpoint for GetRepositoryCommitStat {
    const PATH: &'static str = "/repository/{owner}/{repo}/commits/{sha}/stat";
    const METHOD: http::Method = http::Method::GET;

    type Request = GetRepositoryCommitStatRequest;
    type Response = GetRepositoryCommitStatResponse;
}

#[derive(EndpointRequest, Debug, Serialize, Deserialize)]
pub struct GetRepositoryCommitStatRequest {}

pub type GetRepositoryCommitStatResponse = Vec<RepositoryCommitStatResource>;
