use super::UserEndpointResponse;
use crate::endpoint::{Endpoint, EndpointRequest};
use serde::{Deserialize, Serialize};

pub struct GetUser;

impl Endpoint for GetUser {
    const PATH: &'static str = "/user/{user_name}";
    const METHOD: http::Method = http::Method::GET;

    type Request = GetUserEndpointRequest;
    type Response = UserEndpointResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserEndpointRequest {}

impl EndpointRequest for GetUserEndpointRequest {}
