use serde::{Deserialize, Serialize};
use api_derive::EndpointRequest;

use crate::{endpoint::Endpoint, resource::user::UserResource};

pub struct GetCurrentUser;

impl Endpoint for GetCurrentUser {
    const PATH: &'static str = "/user";
    const METHOD: http::Method = http::Method::GET;

    type Request = GetCurrentUserRequest;
    type Response = GetCurrentUserResponse;
}

#[derive(EndpointRequest, Debug, Serialize, Deserialize)]
pub struct GetCurrentUserRequest {}

pub type GetCurrentUserResponse = UserResource;
