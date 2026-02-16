use serde::{Deserialize, Serialize};

use crate::endpoint::Endpoint;
use crate::resource::user::UserResource;

pub struct GetUser;

impl Endpoint for GetUser {
    const PATH: &'static str = "/user/{user_name}";
    const METHOD: http::Method = http::Method::GET;

    type Request = GetUserRequest;
    type Response = GetUserResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserRequest {}

pub type GetUserResponse = UserResource;
