use crate::{endpoint::Endpoint, resource::migration::GitHubInstallationResource};

pub struct ListGitHubInstallations;

impl Endpoint for ListGitHubInstallations {
    const PATH: &'static str = "/migration/github/installations";
    const METHOD: http::Method = http::Method::GET;

    type Request = ();
    type Response = ListGitHubInstallationsResponse;
}

pub type ListGitHubInstallationsResponse = Vec<GitHubInstallationResource>;
