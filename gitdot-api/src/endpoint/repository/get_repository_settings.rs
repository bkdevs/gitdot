use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::repository::RepositorySettingsResource};

pub struct GetRepositorySettings;

impl Endpoint for GetRepositorySettings {
    const PATH: &'static str = "/repository/{owner}/{repo}/settings";
    const METHOD: http::Method = http::Method::GET;

    type Request = GetRepositorySettingsRequest;
    type Response = GetRepositorySettingsResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct GetRepositorySettingsRequest {}

pub type GetRepositorySettingsResponse = RepositorySettingsResource;
