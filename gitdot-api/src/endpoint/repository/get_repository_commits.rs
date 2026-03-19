use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::default_ref;
use crate::{endpoint::Endpoint, resource::repository::RepositoryCommitsResource};

pub struct GetRepositoryCommits;

impl Endpoint for GetRepositoryCommits {
    const PATH: &'static str = "/repository/{owner}/{repo}/commits";
    const METHOD: http::Method = http::Method::GET;

    type Request = GetRepositoryCommitsRequest;
    type Response = GetRepositoryCommitsResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct GetRepositoryCommitsRequest {
    #[serde(default = "default_ref")]
    pub ref_name: String,
    pub from: Option<DateTime<Utc>>,
    pub to: Option<DateTime<Utc>>,
}

pub type GetRepositoryCommitsResponse = RepositoryCommitsResource;
