use super::UserEndpointResponse;
use crate::endpoint::{Endpoint, EndpointRequest};
use serde::{Deserialize, Serialize};

pub struct UpdateCurrentUser;

impl Endpoint for UpdateCurrentUser {
    const PATH: &'static str = "/user";
    const METHOD: http::Method = http::Method::PATCH;

    type Request = UpdateCurrentUserEndpointRequest;
    type Response = UserEndpointResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCurrentUserEndpointRequest {
    pub name: String,
}

impl EndpointRequest for UpdateCurrentUserEndpointRequest {}
