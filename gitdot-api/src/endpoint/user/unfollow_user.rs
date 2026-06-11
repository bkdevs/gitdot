use serde::{Deserialize, Serialize};

use crate::endpoint::Endpoint;

pub struct UnfollowUser;

impl Endpoint for UnfollowUser {
    const PATH: &'static str = "/user/{user_name}/follow";
    const METHOD: http::Method = http::Method::DELETE;

    type Request = UnfollowUserRequest;
    type Response = UnfollowUserResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct UnfollowUserRequest {}

pub type UnfollowUserResponse = ();
