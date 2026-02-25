use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::user::UserResource};

pub struct UpdateCurrentUser;

impl Endpoint for UpdateCurrentUser {
    const PATH: &'static str = "/user";
    const METHOD: http::Method = http::Method::PATCH;

    type Request = UpdateCurrentUserRequest;
    type Response = UpdateCurrentUserResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct UpdateCurrentUserRequest {
    pub name: String,
}

pub type UpdateCurrentUserResponse = UserResource;
