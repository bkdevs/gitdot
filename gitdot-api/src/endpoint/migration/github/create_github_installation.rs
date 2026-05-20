use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::migration::GitHubInstallationResource};

pub struct CreateGitHubInstallation;

impl Endpoint for CreateGitHubInstallation {
    const PATH: &'static str = "/migration/github/{installation_id}";
    const METHOD: http::Method = http::Method::POST;

    type Request = CreateGitHubInstallationRequest;
    type Response = CreateGitHubInstallationResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct CreateGitHubInstallationRequest {
    pub state: String,
    pub code: String,
}

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreateGitHubInstallationResponse {
    pub installation: GitHubInstallationResource,
    pub action: String,
}
