use crate::endpoint::Endpoint;
use crate::resource::user::UserResource;
use serde::{Deserialize, Serialize};

pub struct UpdateCurrentUser;

impl Endpoint for UpdateCurrentUser {
    const PATH: &'static str = "/user";
    const METHOD: http::Method = http::Method::PATCH;

    type Request = UpdateCurrentUserRequest;
    type Response = UpdateCurrentUserResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCurrentUserRequest {
    pub name: String,
}

pub type UpdateCurrentUserResponse = UserResource;
