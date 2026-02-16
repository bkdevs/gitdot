use serde::{Deserialize, Serialize};

use crate::endpoint::Endpoint;
use crate::resource::repository::RepositoryCommitResource;

pub struct GetRepositoryCommit;

impl Endpoint for GetRepositoryCommit {
    const PATH: &'static str = "/repository/{owner}/{repo}/commits/{sha}";
    const METHOD: http::Method = http::Method::GET;

    type Request = GetRepositoryCommitRequest;
    type Response = GetRepositoryCommitResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRepositoryCommitRequest {}

pub type GetRepositoryCommitResponse = RepositoryCommitResource;
