use crate::{endpoint::Endpoint, resource::migration::GitHubInstallationResource};

pub struct CreateGitHubInstallation;

impl Endpoint for CreateGitHubInstallation {
    const PATH: &'static str = "/migration/github/{installation_id}";
    const METHOD: http::Method = http::Method::POST;

    type Request = ();
    type Response = CreateGitHubInstallationResponse;
}

pub type CreateGitHubInstallationResponse = GitHubInstallationResource;
