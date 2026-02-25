use crate::{endpoint::Endpoint, resource::migration::GitHubRepositoryResource};

pub struct ListGitHubInstallationRepositories;

impl Endpoint for ListGitHubInstallationRepositories {
    const PATH: &'static str = "/migration/github/{installation_id}/repositories";
    const METHOD: http::Method = http::Method::GET;

    type Request = ();
    type Response = ListGitHubInstallationRepositoriesResponse;
}

pub type ListGitHubInstallationRepositoriesResponse = Vec<GitHubRepositoryResource>;
