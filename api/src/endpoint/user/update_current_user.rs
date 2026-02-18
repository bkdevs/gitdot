use serde::{Deserialize, Serialize};
use api_derive::EndpointRequest;

use crate::{endpoint::Endpoint, resource::user::UserResource};

pub struct UpdateCurrentUser;

impl Endpoint for UpdateCurrentUser {
    const PATH: &'static str = "/user";
    const METHOD: http::Method = http::Method::PATCH;

    type Request = UpdateCurrentUserRequest;
    type Response = UpdateCurrentUserResponse;
}

#[derive(EndpointRequest, Debug, Serialize, Deserialize)]
pub struct UpdateCurrentUserRequest {
    pub name: String,
}

pub type UpdateCurrentUserResponse = UserResource;
