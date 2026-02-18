use api_derive::ApiRequest;
use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::user::UserResource};

pub struct GetCurrentUser;

impl Endpoint for GetCurrentUser {
    const PATH: &'static str = "/user";
    const METHOD: http::Method = http::Method::GET;

    type Request = GetCurrentUserRequest;
    type Response = GetCurrentUserResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct GetCurrentUserRequest {}

pub type GetCurrentUserResponse = UserResource;
