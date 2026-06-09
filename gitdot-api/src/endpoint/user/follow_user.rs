use serde::{Deserialize, Serialize};

use crate::endpoint::Endpoint;

pub struct FollowUser;

impl Endpoint for FollowUser {
    const PATH: &'static str = "/user/{user_name}/follow";
    const METHOD: http::Method = http::Method::POST;

    type Request = FollowUserRequest;
    type Response = FollowUserResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct FollowUserRequest {}

pub type FollowUserResponse = ();
