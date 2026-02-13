use super::UserApiResponse;
use crate::endpoint::Endpoint;
use serde::{Deserialize, Serialize};

pub struct UpdateCurrentUser;

impl Endpoint for UpdateCurrentUser {
    const PATH: &'static str = "/user";
    const METHOD: http::Method = http::Method::PATCH;

    type ApiRequest = UpdateCurrentUserApiRequest;
    type ApiResponse = UserApiResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCurrentUserApiRequest {
    pub name: String,
}
