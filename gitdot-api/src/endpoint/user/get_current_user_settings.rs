use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::user::UserSettingsResource};

pub struct GetCurrentUserSettings;

impl Endpoint for GetCurrentUserSettings {
    const PATH: &'static str = "/user/settings";
    const METHOD: http::Method = http::Method::GET;

    type Request = GetCurrentUserSettingsRequest;
    type Response = GetCurrentUserSettingsResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct GetCurrentUserSettingsRequest {}

pub type GetCurrentUserSettingsResponse = UserSettingsResource;
