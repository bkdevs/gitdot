use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    endpoint::Endpoint,
    resource::{settings::CommitFilterResource, user::UserSettingsResource},
};

pub struct UpdateCurrentUserSettings;

impl Endpoint for UpdateCurrentUserSettings {
    const PATH: &'static str = "/user/settings";
    const METHOD: http::Method = http::Method::PATCH;

    type Request = UpdateCurrentUserSettingsRequest;
    type Response = UpdateCurrentUserSettingsResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct UpdateCurrentUserSettingsRequest {
    pub repos: Option<HashMap<String, UpdateUserRepoSettingsRequest>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserRepoSettingsRequest {
    pub commit_filters: Option<Vec<CommitFilterResource>>,
}

pub type UpdateCurrentUserSettingsResponse = UserSettingsResource;
