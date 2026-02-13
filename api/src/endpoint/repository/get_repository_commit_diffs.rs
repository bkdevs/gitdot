use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::repository::RepositoryCommitDiffResource};

pub struct GetRepositoryCommitDiffs;

impl Endpoint for GetRepositoryCommitDiffs {
    const PATH: &'static str = "/repository/{owner}/{repo}/commits/diff";
    const METHOD: http::Method = http::Method::GET;

    type Request = GetRepositoryCommitDiffsRequest;
    type Response = GetRepositoryCommitDiffsResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRepositoryCommitDiffsRequest {
    #[serde(default)]
    pub format: GetRepositoryCommitDiffsFormat,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GetRepositoryCommitDiffsFormat {
    Stats,
    #[default]
    Full,
}

pub type GetRepositoryCommitDiffsResponse = Vec<RepositoryCommitDiffResource>;
