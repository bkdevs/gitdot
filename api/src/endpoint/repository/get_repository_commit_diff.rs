use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::repository::RepositoryCommitDiffResource};

pub struct GetRepositoryCommitDiff;

impl Endpoint for GetRepositoryCommitDiff {
    const PATH: &'static str = "/repository/{owner}/{repo}/commits/{sha}/diff";
    const METHOD: http::Method = http::Method::GET;

    type Request = GetRepositoryCommitDiffRequest;
    type Response = GetRepositoryCommitDiffResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRepositoryCommitDiffRequest {}

pub type GetRepositoryCommitDiffResponse = Vec<RepositoryCommitDiffResource>;
