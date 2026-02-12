use super::UserEndpointResponse;
use crate::endpoint::{Endpoint, EndpointRequest};
use serde::{Deserialize, Serialize};

pub struct GetCurrentUser;

impl Endpoint for GetCurrentUser {
    const PATH: &'static str = "/user";
    const METHOD: http::Method = http::Method::GET;

    type Request = GetCurrentUserEndpointRequest;
    type Response = UserEndpointResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCurrentUserEndpointRequest {}

impl EndpointRequest for GetCurrentUserEndpointRequest {}
