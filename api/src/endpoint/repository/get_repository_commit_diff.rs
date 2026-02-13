use crate::endpoint::Endpoint;
use crate::resource::repository::RepositoryCommitDiffResource;
use serde::{Deserialize, Serialize};

pub struct GetRepositoryCommitDiff;

impl Endpoint for GetRepositoryCommitDiff {
    const PATH: &'static str = "/repository/{owner}/{repo}/commits/{sha}/diff";
    const METHOD: http::Method = http::Method::GET;

    type Request = GetRepositoryCommitDiffRequest;
    type Response = GetRepositoryCommitDiffResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRepositoryCommitDiffRequest {
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

pub type GetRepositoryCommitDiffResponse = Vec<RepositoryCommitDiffResource>;
