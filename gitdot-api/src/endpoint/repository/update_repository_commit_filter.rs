use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::RepositoryCommitFilterResource};

pub struct UpdateRepositoryCommitFilter;

impl Endpoint for UpdateRepositoryCommitFilter {
    const PATH: &'static str = "/repository/{owner}/{repo}/commit_filters/{filter_id}";
    const METHOD: http::Method = http::Method::PATCH;

    type Request = UpdateRepositoryCommitFilterRequest;
    type Response = UpdateRepositoryCommitFilterResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct UpdateRepositoryCommitFilterRequest {
    pub name: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authors: Option<Vec<String>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paths: Option<Vec<String>>,
}

pub type UpdateRepositoryCommitFilterResponse = RepositoryCommitFilterResource;
