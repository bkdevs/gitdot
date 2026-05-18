use serde::{Deserialize, Serialize};

use crate::{
    endpoint::Endpoint,
    resource::{common::Page, migration::GitHubInstallationResource},
};

pub struct ListGitHubInstallations;

impl Endpoint for ListGitHubInstallations {
    const PATH: &'static str = "/migration/github/installations";
    const METHOD: http::Method = http::Method::GET;

    type Request = ListGitHubInstallationsRequest;
    type Response = ListGitHubInstallationsResponse;
}

#[derive(ApiRequest, Debug, Default, Serialize, Deserialize)]
pub struct ListGitHubInstallationsRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

pub type ListGitHubInstallationsResponse = Page<GitHubInstallationResource>;
