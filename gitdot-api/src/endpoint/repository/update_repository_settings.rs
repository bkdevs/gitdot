use serde::{Deserialize, Serialize};

use crate::{
    endpoint::Endpoint,
    resource::{repository::RepositorySettingsResource, settings::CommitFilterResource},
};

pub struct UpdateRepositorySettings;

impl Endpoint for UpdateRepositorySettings {
    const PATH: &'static str = "/repository/{owner}/{repo}/settings";
    const METHOD: http::Method = http::Method::PATCH;

    type Request = UpdateRepositorySettingsRequest;
    type Response = UpdateRepositorySettingsResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct UpdateRepositorySettingsRequest {
    pub commit_filters: Option<Vec<CommitFilterResource>>,
}

pub type UpdateRepositorySettingsResponse = RepositorySettingsResource;
