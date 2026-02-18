use serde::{Deserialize, Serialize};
use api_derive::EndpointRequest;

use crate::{endpoint::Endpoint, resource::user::UserResource};

pub struct GetUser;

impl Endpoint for GetUser {
    const PATH: &'static str = "/user/{user_name}";
    const METHOD: http::Method = http::Method::GET;

    type Request = GetUserRequest;
    type Response = GetUserResponse;
}

#[derive(EndpointRequest, Debug, Serialize, Deserialize)]
pub struct GetUserRequest {}

pub type GetUserResponse = UserResource;
