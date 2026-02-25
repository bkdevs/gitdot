use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::migration::MigrationResource};

pub struct MigrateGitHubRepositories;

impl Endpoint for MigrateGitHubRepositories {
    const PATH: &'static str = "/migration/github/{installation_id}/migrate";
    const METHOD: http::Method = http::Method::POST;

    type Request = MigrateGitHubRepositoriesRequest;
    type Response = MigrateGitHubRepositoriesResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct MigrateGitHubRepositoriesRequest {
    pub origin: String,
    pub origin_type: String,
    pub destination: String,
    pub destination_type: String,
    pub repositories: Vec<String>,
}

pub type MigrateGitHubRepositoriesResponse = MigrationResource;
